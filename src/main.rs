extern crate sdl2;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

use std::{thread, time};
use noise::{NoiseFn, Perlin};
use euclid::*;

mod flowfield;



// array 20 /20
// fn main() {
//     // let par = flowfield::particules::Particule::new();
//     // println!("{:?}", 1);
//     let game = flowfield::Flowfield::new(0);
//     for (_i, unit) in (&game).into_iter().enumerate() {
//         println!("{:?}", unit);
//     }


// }

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window("Flowfield",
                flowfield::SQUARE_SIZE*flowfield::WIDTH,
                flowfield::SQUARE_SIZE*flowfield::HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();
    canvas.set_blend_mode(sdl2::render::BlendMode::Add);

    // this struct manages textures. For lifetime reasons, the canvas cannot directly create
    // textures, you have to create a `TextureCreator` instead.
    // let texture_creator : TextureCreator<_> = canvas.texture_creator();

    // Create a "target" texture so that we can use our Renderer with it later
    // let (square_texture1, square_texture2) = dummy_texture(&mut canvas, &texture_creator)?;
    let mut game = flowfield::Flowfield::new(0);

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame : u32 = 0;
    let mut last_mode = game.mode();
    'running: loop {
        if let flowfield::State::Playing = game.state() {
            last_mode = game.mode();
        }

        // get the inputs here
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    game.toggle_state();
                },
                Event::KeyDown { keycode: Some(Keycode::M), repeat: false, .. } => {
                    game.toggle_mode();
                },
                Event::KeyDown { keycode: Some(Keycode::C), repeat: false, .. } => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                },
                // Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                //     let x = (x as u32) / SQUARE_SIZE;
                //     let y = (y as u32) / SQUARE_SIZE;
                //     match game.get_mut(x as i32, y as i32) {
                //         Some(square) => {*square = !(*square);},
                //         None => unreachable!(),
                //     };
                // },
                _ => {}
            }
        }

        // update the game loop here
        if frame >= 2 {
            // println!("{:?}", "UPDATE");
            game.update();
            frame = 0;
        }

        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.clear();
        if let flowfield::State::Playing = game.state() {




            if let flowfield::Mode::Demo = game.mode() {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                let mut i = 0;
                for line in game.field.iter() {
                    canvas.set_draw_color(Color::RGB((line.cos()*255.0 )as u8, (line.sin()*255.0) as u8, 0));
                    // let pos = Point2D<i32>::new( flowfield::SQUARE_SIZE/2 + i/flowfield::WIDTH, flowfield::SQUARE_SIZE/2 + i%flowfield::WIDTH);
                    // let pos = ((flowfield::SQUARE_SIZE/2 + (i/flowfield::WIDTH)*flowfield::SQUARE_SIZE ) as i32, (flowfield::SQUARE_SIZE/2 + (i%flowfield::WIDTH) * flowfield::SQUARE_SIZE) as i32);
                    let pos = ((flowfield::SQUARE_SIZE/2 + (i%flowfield::WIDTH)*flowfield::SQUARE_SIZE ) as i32, (flowfield::SQUARE_SIZE/2 + (i/flowfield::WIDTH) * flowfield::SQUARE_SIZE) as i32);
                    // let end = Point2D<i32>::new((flowfield::SQUARE_SIZE/2 + i/flowfield::WIDTH) + (line.cos()*10.0) as u32,
                                                    // (flowfield::SQUARE_SIZE/2 + i%flowfield::WIDTH) + (line.sin()*10.0 )as u32));
                    let end = (((flowfield::SQUARE_SIZE/2 + (i%flowfield::WIDTH) * flowfield::SQUARE_SIZE) as i32 + (line.cos()*(flowfield::SQUARE_SIZE/2) as f64) as i32) as i32, 
                                ((flowfield::SQUARE_SIZE/2 + (i/flowfield::WIDTH) * flowfield::SQUARE_SIZE) as i32 + (line.sin()*(flowfield::SQUARE_SIZE/2) as f64)as i32) as i32);
                    // println!("{:?}", line);
                    // println!("{:?}", pos);
                    canvas.draw_line(pos, end)?;
                    i += 1;
                }
                canvas.set_draw_color(Color::RGB(200, 200, 200));
                for (i, part) in (&game).into_iter().enumerate() {
                    canvas.draw_point(part._pos.to_tuple())?;
                }

            }
            else {
                if last_mode == flowfield::Mode::Demo {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                }
                canvas.set_draw_color(Color::RGB(5, 5, 5));
                for (i, part) in (&game).into_iter().enumerate() {

                    // canvas.draw_point(part._pos.to_tuple())?;
                    canvas.draw_line(part._pos.to_tuple(), part._pos_last.to_tuple())?;
                    // let i = i as u32;
                    // let square_texture = if frame >= 15 {
                    //     &square_texture1
                    // } else {
                    //     &square_texture2
                    // };
                    // if *unit {
                    //     canvas.copy(square_texture,
                    //                 None,
                    //                 Rect::new(((i % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                    //                           ((i / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                    //                           SQUARE_SIZE,
                    //                           SQUARE_SIZE))?;
                    // }
                }
            }
            canvas.present();
            frame += 1;
        };
    }

    Ok(())
}




