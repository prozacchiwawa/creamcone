extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use std::time::Duration;

use creamcone::types::{
    CHUNK_SIZE,

    Point,
    IPoint,
    Blop,
    DOFType,
    Object,
    ObjectUniverse,
    ObjectConfiguration,
    Universe
};

fn main() {
    let mut ou = ObjectUniverse::new();
    ou.add(Object::new(
        &"test1".to_string(),
        &Point::new(0.0, 0.0),
        None,
        DOFType::Rotation,
        vec!(
            Blop::new(Point::new(100.0, 0.0), 1.0),
            Blop::new(Point::new(0.0, 100.0), 0.5),
            Blop::new(Point::new(-100.0, 0.0), 2.0),
            Blop::new(Point::new(0.0, -100.0), 3.0)
        )
    ));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let screensize = IPoint::new(800, 600);

    let window = video_subsystem.window(
        "rust-sdl2 demo",
        screensize.x as u32,
        screensize.y as u32
    )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut c = ObjectConfiguration::new(&ou);

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        c.positions[0] += 0.01;
        let u = c.realize(&ou);

        let mut chunkdata = u.tourChunks(|pt,c| (pt.clone(),c.getData()));
        for i in 0..chunkdata.len() {
            let pt = chunkdata[i].0.clone();
            let s = Surface::from_data(
                &mut chunkdata[i].1,
                CHUNK_SIZE as u32,
                CHUNK_SIZE as u32,
                CHUNK_SIZE as u32,
                PixelFormatEnum::RGB332
            ).unwrap();
            let texture = texture_creator.create_texture_from_surface(s).unwrap();
            canvas.copy(
                &texture,
                None,
                Some(Rect::new(
                    pt.x + screensize.x / 2,
                    pt.y + screensize.y / 2,
                    CHUNK_SIZE as u32,
                    CHUNK_SIZE as u32
                ))
            ).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
