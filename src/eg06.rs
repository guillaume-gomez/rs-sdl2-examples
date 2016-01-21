extern crate sdl2;

use sdl2::event::{Event};

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let first_window  = match video_ctx.window("eg06", 640, 480).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // let second_window  = match video_ctx.window("second_window", 640, 480).position_centered().opengl().build() {
    //     Ok(window) => window,
    //     Err(err)   => panic!("failed to create window: {}", err)
    // };

    
    let mut renderer = match first_window.renderer().build() {
         Ok(renderer) => renderer,
         Err(err) => panic!("failed to create renderer: {}", err)
     };
    let _ = renderer.present();

    let mut events = ctx.event_pump().unwrap();
    // loop until we receive a QuitEvent
    'event : loop {
        for event in events.poll_iter() {
            match event {
                Event::Window{window_id,  win_event_id:window_event_id, ..} =>
                    {
                        println!("{:?} ==> {:?}", window_id, window_event_id );
                        match window_event_id {
                            sdl2::event::WindowEventId::Close => break 'event,
                            _ => continue,
                        }
                    },
                _               => continue
            }
        }
    }
}