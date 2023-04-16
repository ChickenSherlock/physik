use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use gtk::{Application, Image, glib, ApplicationWindow,Grid,Overlay};
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
};

const APP_ID: &str = "me.BETTER_PHOTO_BOOTH.com";
const CAMERA_WIDTH: i64 = 1920;
const CAMERA_HEIGHT: i64 = 1080;
const ROW_STRIDE: i64 = CAMERA_WIDTH * 3;

fn create_camera()->Result<(videoio::VideoCapture,Mat)>{
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    Ok((camera,img))
}

fn camera_stream(sender: glib::SyncSender<((Vec<u8>))>){
    thread::spawn(move||{
        let (mut camera, mut mat) =if let Ok((mut camera, mut mat)) =create_camera() { (camera, mat) } else {todo!("")};
        loop {
            camera.read(&mut mat).expect("Camera not available");
            let mut image = Mat::default();
            cvt_color(&mat, &mut image, opencv::imgproc::COLOR_BGR2RGB, 0).unwrap();
            let image_copy = image.clone();
            let mut dest = Mat::default();
            core::flip(&image_copy, &mut dest, 1).unwrap();
            let data = dest.data_bytes().unwrap();
            let message = data.to_vec();
            sender.send(message).expect("Failed");
            thread::sleep(Duration::new(0, 3000000))
        };

    });
}


fn draw_ui(app: &Application) {
    let stream = Image::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let stream_overlay = Overlay::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&stream)
        .build();

    let (pixbuf_sender, pixbuf_receiver) = glib::MainContext::sync_channel(glib::Priority::HIGH,2073600);



    camera_stream(pixbuf_sender);

    pixbuf_receiver.attach(
        None, move|data| {
            let pixbuf = Pixbuf::from_bytes(
                &glib::Bytes::from_owned(data),
                Colorspace::Rgb,
                false,
                8,
                1920,
                1080,
                5760
            );
            stream.set_from_pixbuf(Some(&pixbuf));
            glib::Continue(true)
        }
    );

    let main_box = Grid::builder()
        .build();
    main_box.attach(&stream_overlay,0,1,1,1);
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