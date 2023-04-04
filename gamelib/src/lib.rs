use libc;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;

#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    match my_actual_main_function() {
        Ok(_) => 0i32,
        Err(_) => 1i32,
    }
}

pub fn my_actual_main_function() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Game", 800, 600)
        .fullscreen_desktop()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture: Texture = texture_creator.load_texture("assets/image.jpg")?;

    let q = image_texture.query();
    let rect = Rect::new(0, 0, q.width, q.height);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        canvas.copy(&image_texture, None, rect)?;

        canvas.present();

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
    }

    Ok(())
}
