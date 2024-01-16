mod file_handler;

extern crate sdl2;

use std::time::SystemTime;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;

use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use sdl2::image::{LoadTexture, InitFlag};

const TEXTURE_SIZE: u32 = 32;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
                           texture_creator: &'a TextureCreator<WindowContext>,
                           color: TextureColor, size: u32) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                TextureColor::Green => {
                    texture.set_draw_color(Color::RGB(0, 255, 0));
                }
                TextureColor::Blue => {
                    texture.set_draw_color(Color::RGB(0, 0, 255));
                }
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}


fn main() {
    let sdl_content = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_content.video()
        .expect("Couldn't get SDL video subsystem");

    sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Couldn't initialize image context");

    let window = video_subsystem.window("Tetris", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.
        into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let blue_structure = create_texture_rect(&mut canvas,
                                             &texture_creator, TextureColor::Blue, TEXTURE_SIZE).expect("Failed to create a texture");

    let green_structure = create_texture_rect(&mut canvas,
                                              &texture_creator, TextureColor::Green, TEXTURE_SIZE).expect("Failed to create a texture");

    let image_texture = texture_creator.load_texture("assets/my_image.jpg")
        .expect("Couldn't load image");

    let mut event_pump = sdl_content.event_pump()
        .expect("Failed to get SDL event pump");

    let mut curr_time = SystemTime::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        let square_texture = match curr_time.elapsed() {
            Ok(time) if time.as_secs() % 2 == 0 => {
                &blue_structure
            }
            _ => {
                &green_structure
            }
        };
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();


        canvas.copy(&square_texture,
                    None,
                    Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.copy(&image_texture, None ,None).expect("Render failed");

        canvas.present();
        sleep(Duration::new(0, 1_000_000u32) / 60);
    }
}


