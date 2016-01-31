extern crate sdl2;

use sdl2::event::{Event};
use sdl2::video::{Window};
use sdl2::keyboard::{Keycode};
use sdl2::{VideoSubsystem};


fn build_window(video_ctx: &VideoSubsystem, title: &str) -> (Window) {
    let window = match video_ctx.window(title, 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };
    return window;
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let mut window_ids_vec = Vec::new();
    
    window_ids_vec.push(build_window(&video_ctx, "first_window"));
       
    // let mut renderer = match first_window.renderer().build() {
    //      Ok(renderer) => renderer,
    //      Err(err) => panic!("failed to create renderer: {}", err)
    //  };
    // let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();
    // loop until we receive a QuitEvent 
    while window_ids_vec.len() > 0 {
        for event in events.poll_iter() {
            match event {
                Event::Window{window_id,  win_event_id:window_event_id, ..} =>
                    {
                        println!("{:?} ==> {:?}", window_id, window_event_id );
                        match window_event_id {
                            sdl2::event::WindowEventId::Close =>  
                            { 
                                window_ids_vec.retain(|window| window.id() != window_id)
                            },
                            _ => continue,
                        }
                    },
                Event::KeyDown { keycode: Some(Keycode::N), .. } =>  
                    window_ids_vec.push(build_window(&video_ctx, "another window")) ,
                _               => continue
            }
        }
    }
}