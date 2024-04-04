#[allow(dead_code)]
use raylib::prelude::*;
use raylib::color::Color;
// use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
// use raylib::RaylibHandle::drawing;

pub struct Dash{

    pub visible:bool,
    pub i:i32,
}


impl Dash{
    pub fn draw(&mut self, d: &mut RaylibDrawHandle){
        self.i = 0;
        // d.draw_circle(10,  5, 10.0, Color::RED);
        // d.draw_circle(35,  5, 10.0, Color::YELLOW);
        // d.draw_circle(60,  5, 10.0, Color::ORANGE);

        while self.i < 5{
            d.draw_circle((30 * self.i), 20, 10.0, Color::GREEN);
        // d.draw_circle(60,  5, 10.0, Color::ORANGE);
                // d.draw_circle(10,  5, 10.0, Color::RED);
        // d.draw_circle(35,  5, 10.0, Color::YELLOW);
        // d.draw_circle(60,  5, 10.0, Color::ORANGE);

            self.i = self.i+1;
        }
    }


}

