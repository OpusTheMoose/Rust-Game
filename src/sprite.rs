
use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::video::Window;



pub struct Tile
{
    name: String,
    rect: Rect,
}

pub struct SpriteMap
{
    map: HashMap<String, Texture>
}


impl SpriteMap{
    pub fn new() -> Self {
        return SpriteMap{
            map: HashMap::new(),
        }
    }
    pub fn create_texture(&mut self, canvas : &mut Canvas<Window>, file_path: String )
    {
        if !self.map.contains_key(&file_path)
        {
            println!("Already have texture loaded. ");
            return
        }
        let texture_creator = canvas.texture_creator();
        let texture_ = texture_creator.load_texture("32x32.png").unwrap();
       
         self.map.insert(file_path, texture_);
    }
    pub fn get_texture(self, name: String) -> Texture
    {
        return self.map.get(&name).unwrap();
    }
}
impl Tile{
    pub fn new(rect_: Rect, name_: String) -> Self{
        return Tile {
            rect: rect_,
            name: name_,
        }
    }
}
