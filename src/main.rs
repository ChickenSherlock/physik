use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::borrow::Borrow;
use std::path::Path;
use gtk::{Application, Image, glib, ApplicationWindow, Grid, Overlay, Button, Label, CssProvider, StyleContext, Scrollable, ScrolledWindow};
use gdk_pixbuf::{Pixbuf,Colorspace};
use timer;
use gtk::gio::ffi::G_NOTIFICATION_PRIORITY_HIGH;
use gtk::glib::{Continue, MainContext, Priority};
use std::time::{Duration, Instant};
use gtk::prelude::*;
use serde::Deserialize;
use serde_json;

use opencv::{
    Result,
    prelude::*,
    imgproc::cvt_color,
    imgcodecs,
    types,
    videoio,
    core,
    imgproc::resize,
    imgcodecs::imwrite
};

#[derive(Debug, Deserialize)]
struct Auth {
    first_name: String,
    last_name: String,
    class: String,
    uuid: i32
}
#[derive(Debug, Deserialize)]
struct AuthList{
    access: Vec<Auth>
}

use std::sync::atomic::{AtomicBool, Ordering};
use gtk::gdk::Screen;
use crate::glib::Sender;

//camera output settings and app settings
const APP_ID: &str = "me.BETTER_PHOTO_BOOTH.com";
const CAMERA_WIDTH: i32 = 1280;
const CAMERA_HEIGHT: i32 = 720;
const ROW_STRIDE: i32 = CAMERA_WIDTH * 3;

fn create_camera()->Result<(videoio::VideoCapture,Mat)>{
    let mut camera = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut img = Mat::default();
    Ok((camera,img))
}

fn camera_stream(sender: glib::Sender<((Vec<u8>))>, cambool: Arc<AtomicBool>, uuid_sender: glib::Sender<String>, authbool_clone: Arc<AtomicBool>){
    thread::spawn(move||{
        let (mut camera, mut mat) =if let Ok((mut camera, mut mat)) =create_camera() { (camera, mat) } else {todo!("")};
        loop {
            if !cambool.load(Ordering::Relaxed){
                // reading camera footage
                camera.read(&mut mat).expect("Camera not available");
                // calling decode function every second
                // some basic image manipulation
                let mut image = Mat::default();
                cvt_color(&mat, &mut image, opencv::imgproc::COLOR_BGR2RGB, 0).unwrap();
                let image_copy = image.clone();
                let mut dest = Mat::default();
                core::flip(&image_copy, &mut dest, 1).unwrap();
                let mut resized_dest = Mat::default();
                resize(&dest,&mut resized_dest,core::Size::new(1280,720),0.0,0.0,0);
                let data = resized_dest.data_bytes().unwrap();
                let message = data.to_vec();

                //sending Image as vec[u8] to main thread
                sender.send(message.clone()).expect("Failed");
                // sleeping thread as we dont want to read the camera all the time
                thread::sleep(Duration::new(0, 3000000))
            }
        };
    });
}


// opens access.json and gets information for all the people that are allowed to do images
fn get_access_information() -> AuthList{
    let auth_file = fs::File::open("./src/access.json").expect("unable to open file");
    let auth_data:AuthList = serde_json::from_reader(auth_file).expect("unable to deserialize");
    return auth_data
}


fn draw_ui(app: &Application) {
    let auth_data = get_access_information();


    // main atomic bool that shuts down camera in camera_stream thread allows other threads to access camera
    let cambool = Arc::new(AtomicBool::new(false));
    let cambool_1 = cambool.clone();
    let cambool_5 = cambool.clone();

    let authbool = Arc::new(AtomicBool::new(false));
    let authbool_clone = authbool.clone();
    let abc = authbool.clone();
    let abc_ =authbool.clone();
    let au = authbool.clone();


    let stream = Image::builder()
        .hexpand(true)
        .vexpand(true)
        .build();



    let grid = Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .row_spacing(600)
        .build();

    let button_1 = Button::builder()
        .label("Herr Detlefsen")
        .build();

    let button_2 = Button::builder()
        .label("Beni")
        .build();






    let overlay_label = Label::builder()
        .label("")
        .name("label")
        .build();




    let overlay = Overlay::builder()
        .child(&stream)
        .build();

    let overlay_clone = overlay.clone();
    grid.attach(&button_2,1,1,1,1);
    grid.attach(&button_1,0,1,1,1);
    grid.attach(&overlay_label,0,0,1,1);


    overlay.add_overlay(&grid);
    let choose_grid = Grid::builder()
        .hexpand(true)
        .vexpand(true)
        .build();


    overlay.add_overlay(&grid);

    // main channel that allows communication with camera_stream thread and main ui thread
    let (pixbuf_sender, pixbuf_receiver) = glib::MainContext::channel(glib::Priority::HIGH);

    let (finished_sender,finished_receiver) = glib::MainContext::channel(Priority::HIGH);
    // main channel that allows uuids to be passed to main ui thread
    let (finished_sender_,finished_receiver_) = glib::MainContext::channel(Priority::HIGH);

    let (uuid_sender,uuid_receiver) = glib::MainContext::channel(glib::Priority::HIGH);

    // starts camera_stream thread
    camera_stream(pixbuf_sender,cambool_1,uuid_sender,authbool_clone);

    let main_box = Grid::builder()
        .build();

    main_box.attach(&overlay,0,1,1,1);
    let main_box_clone = main_box.clone();
    let main_box_clone_ = main_box.clone();
    let o_label_clone = overlay_label.clone();

    button_1.connect_button_press_event(move|_,_|{
        let uuid = "Herr-Detlefsen";
        let ol_clone = o_label_clone.clone();
        let u = ol_clone.clone();
        let cambool_3 = cambool.clone();
        let person_sender = finished_sender.clone();
        let auth_list = get_access_information();


        au.store(true,Ordering::Relaxed);
        ol_clone.set_label("Fotos werden in 3 sekunden gemacht");
        glib::timeout_add_local_once(Duration::from_secs(4),move||{
            let l = ol_clone.clone();
            l.set_label("Jetzt gehts los");
        });

        glib::timeout_add_local_once(Duration::from_secs(5),move||{
            let a = u.clone();
            a.set_label("");
        });
        let thread = thread::spawn(move||{
            thread::sleep(Duration::from_secs(7));
            let finished_sender_clone = person_sender.clone();
            for i in 0..3{
                let cambool_2 = cambool_3.clone();
                cambool_2.store(true,Ordering::Relaxed);
                let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Error");
                let mut mat = Mat::default();
                cam.read(&mut mat).expect("Camera not available");
                cambool_2.store(false,Ordering::Relaxed);
                let mut dest = Mat::default();
                let empty_array:opencv::core::Vector<i32> = opencv::core::Vector::new();
                core::flip(&mat, &mut dest, 1).unwrap();
                imwrite(&format!("/Users/benediktkarli/Desktop/physik/v2/src/{:?}-{:?}.jpg",uuid.to_owned(),i).to_owned(),&dest,&empty_array);
                thread::sleep(Duration::from_secs(1))
            }
            finished_sender_clone.send(uuid)
        });
        Inhibit(true)
    });

    let u = overlay_label.clone();

    button_2.connect_button_press_event(move|_,_|{
        let uuid = "Benedikt";
        let ol_clone = u.clone();
        let u = ol_clone.clone();
        let cambool_3 = cambool_5.clone();
        let person_sender = finished_sender_.clone();
        let auth_list = get_access_information();


        authbool.store(true,Ordering::Relaxed);
        ol_clone.set_label("Fotos werden in 3 sekunden gemacht");
        glib::timeout_add_local_once(Duration::from_secs(4),move||{
            let l = ol_clone.clone();
            l.set_label("Jetzt gehts los");
        });

        glib::timeout_add_local_once(Duration::from_secs(5),move||{
            let a = u.clone();
            a.set_label("");
        });
        let thread = thread::spawn(move||{
            thread::sleep(Duration::from_secs(7));
            let finished_sender_clone = person_sender.clone();
            for i in 0..3{
                let cambool_2 = cambool_3.clone();
                cambool_2.store(true,Ordering::Relaxed);
                let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Error");
                let mut mat = Mat::default();
                cam.read(&mut mat).expect("Camera not available");
                cambool_2.store(false,Ordering::Relaxed);
                let mut dest = Mat::default();
                let empty_array:opencv::core::Vector<i32> = opencv::core::Vector::new();
                core::flip(&mat, &mut dest, 1).unwrap();
                imwrite(&format!("/Users/benediktkarli/Desktop/physik/v2/src/{:?}-{:?}.jpg",uuid.to_owned(),i).to_owned(),&dest,&empty_array);
                thread::sleep(Duration::from_secs(1))
            }
            finished_sender_clone.send(uuid)
        });
        Inhibit(true)
    });



    let mb = main_box.clone();
    let cg = choose_grid.clone();
    let o = overlay.clone();
    let ol = overlay_label.clone();

    let mb_ = main_box.clone();
    let cg_ = choose_grid.clone();
    let o_ = overlay.clone();
    let ol_ = overlay_label.clone();
    finished_receiver_.attach(None,move|uuid: &str|{
        for i in 0..3{
            let t = ol.clone();
            let u = mb.clone();
            let ui = abc.clone();
            let e = cg.clone();
            let z = o.clone();
            let image = Image::builder()
                .pixbuf(&Pixbuf::from_file_at_scale(<std::path::Path as AsRef<Path>>::as_ref(Path::new(&format!("/Users/benediktkarli/Desktop/physik/v2/src/{:?}-{:?}.jpg",uuid,i)).as_ref()),426,239,false).unwrap())
                .vexpand(true)
                .hexpand(true)
                .build();

            let button = Button::builder()
                .label(format!("Bild: {:?}",i + 1))
                .margin_bottom(100)
                .build();
            button.set_widget_name(i.to_string().borrow());

            button.connect_button_press_event(move|a,event|{
                std::fs::copy(<std::path::Path as AsRef<Path>>::as_ref(Path::new(&format!("/Users/benediktkarli/Desktop/physik/v2/src/{:?}-{:?}.jpg",uuid,i)).as_ref()),<std::path::Path as AsRef<Path>>::as_ref(Path::new(&format!("/Users/benediktkarli/Desktop/physik/v2/src/output/{:?}.jpg",uuid)).as_ref()));
                let i = u.clone();
                let x = ui.clone();
                let y = t.clone();
                y.set_label("Scanne deinen QR Code");
                i.remove(&e);
                i.attach(&z,0,0,1,1);
                x.store(false,Ordering::Relaxed);
                Inhibit(true)
            });
            choose_grid.attach(&image,i,0,1,1);
            choose_grid.attach(&button,i,1,1,1);
        }
        main_box_clone.remove(&overlay_clone);
        choose_grid.show_all();
        main_box_clone.attach(&choose_grid,0,0,1,1);

        glib::Continue(true)
    });

    finished_receiver.attach(None,move|uuid: &str|{
        for i in 0..3{
            let t = ol_.clone();
            let u = mb_.clone();
            let ui = abc_.clone();
            let e = cg_.clone();
            let z = o_.clone();
            let image = Image::builder()
                .pixbuf(&Pixbuf::from_file_at_scale(<std::path::Path as AsRef<Path>>::as_ref(Path::new(&format!("/Users/benediktkarli/Desktop/physik/v2/src/{:?}-{:?}.jpg",uuid,i)).as_ref()),426,239,false).unwrap())
                .vexpand(true)
                .hexpand(true)
                .build();

            let button = Button::builder()
                .label(format!("Bild: {:?}",i + 1))
                .margin_bottom(100)
                .build();
            button.set_widget_name(i.to_string().borrow());

            button.connect_button_press_event(move|a,event|{
                std::fs::copy(<std::path::Path as AsRef<Path>>::as_ref(Path::new(&format!("/Users/benediktkarli/Desktop/physik/v2/src/{:?}-{:?}.jpg",uuid,i)).as_ref()),<std::path::Path as AsRef<Path>>::as_ref(Path::new(&format!("/Users/benediktkarli/Desktop/physik/v2/src/output/{:?}.jpg",uuid)).as_ref()));
                let i = u.clone();
                let x = ui.clone();
                let y = t.clone();
                y.set_label("Scanne deinen QR Code");
                i.remove(&e);
                i.attach(&z,0,0,1,1);
                x.store(false,Ordering::Relaxed);
                Inhibit(true)
            });
            cg_.attach(&image,i,0,1,1);
            cg_.attach(&button,i,1,1,1);
        }
        main_box_clone_.remove(&overlay);
        cg_.show_all();
        main_box_clone_.attach(&cg_,0,0,1,1);

        glib::Continue(true)
    });
    /*button.connect_button_press_event(move|_,_|{
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
    });*/

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
fn load_css(){
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css").as_bytes());
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("failed"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    )
}

fn main(){

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(draw_ui);
    app.run();
}