extern crate sdl2;

use sdl2::Sdl;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use sdl2::EventPump;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;



pub struct App{
    pub window: Window,
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
   // pub texture_creator: TextureCreator<WindowContext>,
    pub event_pump: EventPump,
    

}

impl App{
    pub fn render(&mut self)
    {
        //let step_size = 16;
        let mut accum = 0.0;
        'running: loop {
            let current = Instant::now();
            self.canvas.set_draw_color(Color::RGB(0, 255, 255));
            self.canvas.clear();
            for event in self.event_pump.poll_iter() {
             match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
            // The rest of the game loop goes here...
           
            //self.canvas.copy(&sprite_.texture, sprite_.src, sprite_.dst).unwrap();
            self.canvas.present();

            let dt = current.elapsed();
            accum += dt.as_secs_f32();
            if accum > 1.0
            {
                //println!("Rendering took: {} ms.", dt.as_secs_f32() * 1000.0);
                println!("FPS: {} ", 1.0 / dt.as_secs_f32());
                accum = 0.0;
            }
        }
    }

    pub fn create_app() -> Self
    {
        let sdl_context_ = sdl2::init().unwrap();
        let video_subsystem_ = sdl_context_.video().unwrap();

        let window_ = video_subsystem_.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

        let canvas_ = window_.clone().into_canvas().build().unwrap();
        let event_pump_ = sdl_context_.event_pump().unwrap();

        return App {
            window: window_,
            canvas: canvas_,
            sdl_context: sdl_context_,
            event_pump: event_pump_,
            }
    }
 


}
