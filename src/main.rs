extern crate raylib;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Bad Apple with Raylib & Rust")
        .build();

    let mut frame = 0;

    let mut img = Image::load_image("assets/images/" + frame + ".png").expect("could not load image " + frame);
    
    let t = rl
        .load_texture_from_image(&thread, &img)
        .expect("could not load texture from image");

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&t, 0, 0, Color::WHITE);

        frame++;
    }
}