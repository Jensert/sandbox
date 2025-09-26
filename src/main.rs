use macroquad::{main, prelude::*};
mod lib;
use lib::{App, window_settings};

#[main(window_settings)]
async fn main() {
    let mut app = App::new((160, 90));
    while app.running() {
        app.handle_input();
        app.start_drawing();
        clear_background(SKYBLUE);

        app.pixels().draw();
        app.pixels_mut().update();

        app.stop_drawing();
        next_frame().await;
    }
}
