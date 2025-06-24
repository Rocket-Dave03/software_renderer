use rgb::{FromSlice, RGB8};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let tc = canvas.texture_creator();
    let mut tex = tc
        .create_texture(
            Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
            sdl2::render::TextureAccess::Streaming,
            800,
            600,
        )
        .expect("failed to create texture");

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        tex.with_lock(None, |pixels: &mut [u8], stride: usize| {
            let view: &mut [u32] = unsafe { std::slice::from_raw_parts_mut(std::mem::transmute(pixels.as_mut_ptr()), pixels.len()/4) };
            for y in 0..600 {
                for x in 0..800 {
                    let idx = y * stride / 4 + x;
                    let color: (u8,u8,u8,u8)  = (x as u8,y as u8,0,0);
                    let int: u32 = unsafe { std::mem::transmute(color) };
                    view[idx] = int.to_be();
                }
            }
        })
        .unwrap();
        canvas.copy(&tex, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
