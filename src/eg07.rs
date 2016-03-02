extern crate sdl2;

use std::path::Path;

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::render::{Texture, Renderer};
use sdl2::rect::{Rect};

pub struct Sprite {
    tex: Texture,
    filename: String
}

impl Sprite {
    pub fn new(f: &str, renderer: &mut Renderer) -> Sprite {
        //Load a surface.
        //Surfaces live in system RAM, so they aren't ideal for performance.
        let surface = match Surface::load_bmp(&Path::new("res/ice-troll.bmp")) {
            Ok(surface) => surface,
            Err(err)    => panic!("failed to load surface: {}", err)
        };

        let texture = match renderer.create_texture_from_surface(&surface) {
                Ok(texture) => texture,
                Err(err)    => panic!("failed to convert surface: {}", err)
            };

        Sprite {
            filename: f.to_owned(),
            tex: texture
        }
    }
}



fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx.window("eg07", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };


    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let test = Sprite::new("jkfkjfdkj", &mut renderer);

    let _ = renderer.clear();
    // Display the texture.
    // Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
    // Try passing Some(surface.get_rect()) for src & dst instead of None & see how things change.
    let _ = renderer.copy(&test.tex, None, Rect::new(100, 100, 128, 128).unwrap());
    let _ = renderer.present();
    
    let mut events = ctx.event_pump().unwrap();

    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
    }
}

