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
    Universe,

    update_simulation,
    create_field_vec
};

fn main() {
    let mut ou = ObjectUniverse::new();
    ou.add(Object::new(
        &"test1".to_string(),
        &Point::new(0.0, 0.0),
        None,
        DOFType::Rotation,
        vec!(
            Blop::new(Point::new(100.0, 0.0), 10.0),
            Blop::new(Point::new(60.0, 60.0), 10.0),
            Blop::new(Point::new(0.0, 100.0), 10.0),
            Blop::new(Point::new(-60.0, 60.0), 10.0),
            Blop::new(Point::new(-100.0, 0.0), 10.0),
            Blop::new(Point::new(60.0, -60.0), 10.0),
            Blop::new(Point::new(0.0, -100.0), 10.0),
            Blop::new(Point::new(-60.0, -60.0), 10.0),
        )
    ));
    ou.add(Object::new(
        &"test2".to_string(),
        &Point::new(-197.0, 10.0),
        None,
        DOFType::Rotation,
        vec!(
            Blop::new(Point::new(95.0, 0.0), 10.0),
            Blop::new(Point::new(100.0, 0.0), 10.0),
            Blop::new(Point::new(105.0, 0.0), 10.0),
            Blop::new(Point::new(110.0, 0.0), 10.0),
            Blop::new(Point::new(115.0, 0.0), 10.0),
            Blop::new(Point::new(120.0, 0.0), 10.0),
            Blop::new(Point::new(125.0, 0.0), 10.0),
            Blop::new(Point::new(0.0, 95.0), 10.0),
            Blop::new(Point::new(0.0, 100.0), 10.0),
            Blop::new(Point::new(0.0, 105.0), 10.0),
            Blop::new(Point::new(0.0, 110.0), 10.0),
            Blop::new(Point::new(0.0, 115.0), 10.0),
            Blop::new(Point::new(0.0, 120.0), 10.0),
            Blop::new(Point::new(0.0, 125.0), 10.0),
            Blop::new(Point::new(-95.0, 0.0), 10.0),
            Blop::new(Point::new(-100.0, 0.0), 10.0),
            Blop::new(Point::new(-105.0, 0.0), 10.0),
            Blop::new(Point::new(-110.0, 0.0), 10.0),
            Blop::new(Point::new(-115.0, 0.0), 10.0),
            Blop::new(Point::new(-120.0, 0.0), 10.0),
            Blop::new(Point::new(-125.0, 0.0), 10.0),
            Blop::new(Point::new(0.0, -95.0), 10.0),
            Blop::new(Point::new(0.0, -100.0), 10.0),
            Blop::new(Point::new(0.0, -105.0), 10.0),
            Blop::new(Point::new(0.0, -110.0), 10.0),
            Blop::new(Point::new(0.0, -115.0), 10.0),
            Blop::new(Point::new(0.0, -120.0), 10.0),
            Blop::new(Point::new(0.0, -125.0), 10.0),
            Blop::new(Point::new(47.5, 82.27), 10.0),
            Blop::new(Point::new(50.0, 86.6), 10.0),
            Blop::new(Point::new(52.5, 90.3), 10.0),
            Blop::new(Point::new(55.0, 95.26), 10.0),
            Blop::new(Point::new(57.5, 99.95), 10.0),
            Blop::new(Point::new(60.0, 103.92), 10.0),
            Blop::new(Point::new(82.27, 47.5), 10.0),
            Blop::new(Point::new(86.6, 50.0), 10.0),
            Blop::new(Point::new(90.3, 52.5), 10.0),
            Blop::new(Point::new(95.26, 55.0), 10.0),
            Blop::new(Point::new(99.95, 57.5), 10.0),
            Blop::new(Point::new(103.92, 60.0), 10.0),
            Blop::new(Point::new(-47.5, 82.27), 10.0),
            Blop::new(Point::new(-50.0, 86.6), 10.0),
            Blop::new(Point::new(-52.5, 90.3), 10.0),
            Blop::new(Point::new(-55.0, 95.26), 10.0),
            Blop::new(Point::new(-57.5, 99.95), 10.0),
            Blop::new(Point::new(-60.0, 103.92), 10.0),
            Blop::new(Point::new(82.27, -47.5), 10.0),
            Blop::new(Point::new(86.6, -50.0), 10.0),
            Blop::new(Point::new(90.3, -52.5), 10.0),
            Blop::new(Point::new(95.26, -55.0), 10.0),
            Blop::new(Point::new(99.95, -57.5), 10.0),
            Blop::new(Point::new(103.92, -60.0), 10.0),
            Blop::new(Point::new(47.5, -82.27), 10.0),
            Blop::new(Point::new(50.0, -86.6), 10.0),
            Blop::new(Point::new(52.5, -90.3), 10.0),
            Blop::new(Point::new(55.0, -95.26), 10.0),
            Blop::new(Point::new(57.5, -99.95), 10.0),
            Blop::new(Point::new(60.0, -103.92), 10.0),
            Blop::new(Point::new(-82.27, 47.5), 10.0),
            Blop::new(Point::new(-86.6, 50.0), 10.0),
            Blop::new(Point::new(-90.3, 52.5), 10.0),
            Blop::new(Point::new(-95.26, 55.0), 10.0),
            Blop::new(Point::new(-99.95, 57.5), 10.0),
            Blop::new(Point::new(-103.92, 60.0), 10.0),
            Blop::new(Point::new(-47.5, -82.27), 10.0),
            Blop::new(Point::new(-50.0, -86.6), 10.0),
            Blop::new(Point::new(-52.5, -90.3), 10.0),
            Blop::new(Point::new(-55.0, -95.26), 10.0),
            Blop::new(Point::new(-57.5, -99.95), 10.0),
            Blop::new(Point::new(-60.0, -103.92), 10.0),
            Blop::new(Point::new(-82.27, -47.5), 10.0),
            Blop::new(Point::new(-86.6, -50.0), 10.0),
            Blop::new(Point::new(-90.3, -52.5), 10.0),
            Blop::new(Point::new(-95.26, -55.0), 10.0),
            Blop::new(Point::new(-99.95, -57.5), 10.0),
            Blop::new(Point::new(-103.92, -60.0), 10.0),
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
    let mut momentum = ObjectConfiguration::new(&ou);
    let field_ref = create_field_vec();

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
        c.positions[0] += 0.005;
        c = c.plus(&momentum);
        let new_c = update_simulation(&field_ref, &c, &ou);
        let momentum = new_c.minus(&c);
        c = new_c;
        let u = c.realize(&field_ref, &ou);

        let mut chunkdata = u.tourChunks(|pt,c| (pt.clone(),c.getData()));
        for i in 0..chunkdata.len() {
            let pt = chunkdata[i].0.clone();
            let s = Surface::from_data(
                &mut chunkdata[i].1,
                CHUNK_SIZE as u32,
                CHUNK_SIZE as u32,
                (CHUNK_SIZE * 4) as u32,
                PixelFormatEnum::ARGB32,
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
    }
}
