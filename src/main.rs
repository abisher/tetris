mod file_handler;
mod tetrimino;
mod tetris_struct;

extern crate sdl2;

use std::time::SystemTime;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::EventPump;

use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use sdl2::image::{LoadTexture, InitFlag};

use tetris_struct::{Tetris};
use file_handler::{load_highscores_and_lines, save_highscores};

const TEXTURE_SIZE: u32 = 32;
const NB_HIGHSCORES: usize = 5;

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

fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime,
                 event_pump: &mut EventPump) -> bool {
    let mut make_permanent = false;

    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *quit = true;
                    break;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    *timer = SystemTime::now();
                    break;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    tmp_x += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    tmp_x -= 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let x = piece.x;
                    let mut y = piece.y;

                    while piece.change_position(&tetris.game_map, x, y + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
            if !make_permanent {
                if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false
                    && tmp_y != piece.y {
                    make_permanent = true;
                }
            }
        }
        if make_permanent {
            tetris.make_permanent();
        }
    }
    make_permanent
}

fn print_game_info(tetris: &mut Tetris) {
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;

    if let Some((mut highscores, mut lines_sent)) =
        load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores, tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent, tetris.nb_lines);

        if new_highest_lines_sent || new_highest_lines_sent {
            save_highscores(&highscores, &lines_sent);
        }
    } else {
        save_highscores(&[tetris.score], &[tetris.nb_lines]);
    }


    println!("Game over...");
    println!("Score: {}{}",
             tetris.score,
             if new_highest_highscore { " [NEW HIGHSCORE]" } else {
                 ""
             });
    println!("Number of lines: {}{}",
             tetris.nb_lines,
             if new_highest_lines_sent { " [NEW HIGHSCORE]" } else {
                 ""
             });
    println!("Current level:      {}", tetris.current_level);
}

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NB_HIGHSCORES {
        v.push(value);
        v.sort();
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
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


    let mut event_pump = sdl_content.event_pump()
        .expect("Failed to get SDL event pump");


    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    loop {
        if match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() >= 1,
            Err(_) => false
        } {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }

            timer = SystemTime::now();
        }
        // Drawing tetris
        if tetris.current_piece.is_none() {
            let current_piece = tetris.create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_info(&mut tetris);
                break;
            }
            tetris.current_piece = Some(current_piece);
        }
        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                // Drawing current tetrimino here
            }
        }
        if quit {
            print_game_info(&mut tetris);
            break;
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        canvas.present();
        sleep(Duration::new(0, 1_000_000u32) / 60);
    }
}
