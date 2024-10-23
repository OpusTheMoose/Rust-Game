extern crate sdl2;

use crate::app::App;

//use crate::app::create_app;

pub mod sprite;
pub mod app;

pub fn main() {
   
    let mut app= App::create_app();
    app.render();
    
}