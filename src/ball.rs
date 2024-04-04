#[allow(dead_code)]
use raylib::color::Color;
use raylib::prelude::*;
// use crate::ffi::Vector2;
// use vector2::Vector2;
// use raylib::math::Vector2;

pub struct Ball{
    pub position: Vector2,
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
}

impl Ball{

    pub fn change_color(&mut self){
        self.color = Color::WHITE;

    }


    pub fn draw(&self, d: &mut RaylibDrawHandle){
        d.draw_circle(self.position.x as i32 + 50,  (self.position.y + 60.0f32)as i32, self.radius, self.color);
        
    }

}