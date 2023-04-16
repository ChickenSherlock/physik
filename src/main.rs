use gtk::{Application};

const APP_ID: &str = "BETTER_PHOTO_BOOTH";
const CAMERA_WIDTH: i64 = 1920;
const CAMERA_HEIGHT: i64 = 1080;
const ROW_STRIDE: i64 = CAMERA_WIDTH * 3;

fn read_camera() {

}

fn draw_ui() {

}


fn main(){
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
}