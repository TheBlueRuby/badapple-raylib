extern crate raylib;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(960, 720)
        .title("Bad Apple with Raylib & Rust")
        .msaa_4x()
        .resizable()
        .build();
    
    hide_console_window();

    let mut frame = 0;

    let mut animation_playing = true;
    rl.set_target_fps(10);
    while !rl.window_should_close() && animation_playing {
        frame += 1;
        let frame_str = format!("{:04}", frame);
        println!("{}", frame_str);

        let img = Image::load_image(&("assets/images/".to_owned() + frame_str.as_str() + ".png"))
            .expect(&("could not load image ".to_owned() + frame_str.as_str() + ".png"));

        let t = rl
            .load_texture_from_image(&thread, &img)
            .expect("could not load texture from image");

        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(&t, Vector2{x: 0.0, y: 0.0}, 0.0, 2.0, Color::WHITE);

        if frame >= 2191 {
            animation_playing = false;
        }
    }
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::{GetConsoleWindow, FreeConsole};
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    println!("Hiding console window!");

    let window = unsafe {GetConsoleWindow()};
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
    
    unsafe { FreeConsole() };
}
