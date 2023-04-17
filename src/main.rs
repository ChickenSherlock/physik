use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use gtk::{Application, Image, glib, ApplicationWindow, Grid, Overlay, Button};
use gdk_pixbuf::{Pixbuf,Colorspace};
use gtk::gio::ffi::G_NOTIFICATION_PRIORITY_HIGH;
use gtk::glib::{Continue, MainContext, Priority};
use gtk::prelude::*;
use opencv::{
    Result,
    prelude::*,
    objdetect,
    imgproc::cvt_color,
    imgcodecs,
    types,
    videoio,
    core,
    imgcodecs::imwrite
};

use std::sync::atomic::{AtomicBool, Ordering};
use crate::glib::Sender;


const APP_ID: &str = "me.BETTER_PHOTO_BOOTH.com";
const CAMERA_WIDTH: i32 = 1280;
const CAMERA_HEIGHT: i32 = 720;
const ROW_STRIDE: i32 = CAMERA_WIDTH * 3;

fn create_camera()->Result<(videoio::VideoCapture,Mat)>{
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    Ok((camera,img))
}

fn camera_stream(sender: glib::Sender<((Vec<u8>))>, cambool: Arc<AtomicBool>){

    thread::spawn(move||{
        let (mut camera, mut mat) =if let Ok((mut camera, mut mat)) =create_camera() { (camera, mat) } else {todo!("")};

        loop {
            if !cambool.load(Ordering::Relaxed){
                camera.read(&mut mat).expect("Camera not available");
                let mut image = Mat::default();
                cvt_color(&mat, &mut image, opencv::imgproc::COLOR_BGR2RGB, 0).unwrap();
                let image_copy = image.clone();
                let mut dest = Mat::default();
                core::flip(&image_copy, &mut dest, 1).unwrap();
                let data = dest.data_bytes().unwrap();
                let message = data.to_vec();
                sender.send(message).expect("Failed");
            }thread::sleep(Duration::new(0, 3000000))

        };

    });
}


fn draw_ui(app: &Application) {
    let cambool = Arc::new(AtomicBool::new(false));
    let cambool_1 = cambool.clone();


    let stream = Image::builder()
        .hexpand(true)
        .vexpand(true)
        .build();


    let button = Button::builder()
        .label("Foto aufnehmen")
        .hexpand(false)
        .vexpand(false)
        .width_request(100)
        .height_request(100)
        .build();

    let grid = Grid::builder()
        .margin_top(CAMERA_HEIGHT-100)
        .margin_start((CAMERA_WIDTH/2)-50)
        .build();
    grid.attach(&button,1,1,1,1);

    let overlay = Overlay::builder()
        .child(&stream)
        .build();


    overlay.add_overlay(&grid);

    let stream_clone = stream.clone();


    let (pixbuf_sender, pixbuf_receiver) = glib::MainContext::channel(glib::Priority::HIGH);



    camera_stream(pixbuf_sender,cambool_1);

    button.connect_button_press_event(move|_,_|{

        let cambool_2 = cambool.clone();
        thread::spawn(move||{
            cambool_2.store(true,Ordering::Relaxed);
            let (mut camera, mut mat) =if let Ok((mut camera, mut mat)) =create_camera() { (camera, mat) } else {todo!("")};
            thread::sleep(Duration::from_secs(3));
            camera.read(&mut mat).expect("Camera not available");
            let mut dest = Mat::default();
            let empty_array:opencv::core::Vector<i32> = opencv::core::Vector::new();
            core::flip(&mat, &mut dest, 1).unwrap();
            imwrite("/Users/benediktkarli/Desktop/physik/v2/src/test.png",&dest,&empty_array);
            cambool_2.store(false,Ordering::Relaxed);
        });
        Inhibit(true)
    });

    pixbuf_receiver.attach(
        None, move|data| {
            let pixbuf = Pixbuf::from_bytes(
                &glib::Bytes::from_owned(data),
                Colorspace::Rgb,
                false,
                8,
                CAMERA_WIDTH,
                CAMERA_HEIGHT,
                ROW_STRIDE
            );
            stream.set_from_pixbuf(Some(&pixbuf));
            glib::Continue(true)
        }
    );

    let main_box = Grid::builder()
        .build();
    main_box.attach(&overlay,0,1,1,1);





    let window = ApplicationWindow::builder()
        .application(app)
        .resizable(false)
        .default_height(720)
        .default_width(1280)
        .child(&main_box)
        .title("Physik")
        .build();

    window.show_all();

}


fn main(){
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(draw_ui);
    app.run();
}