use dash::Dash;
use raylib::prelude::*;
mod dash;
//use dash::dash::*;
mod ball;

use crate::ball::Ball;
use raylib::consts::KeyboardKey::*;

const SCREEN_HEIGHT:f32 = 400.0;
const SCREEN_WIDTH:f32= 800.0;

fn main() {
    // println!(&c);
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello, World")
        .vsync()
        .build();

        rl.set_target_fps(60);
        let mut ball = Ball{
        position: Vector2::new(SCREEN_WIDTH/2f32, SCREEN_HEIGHT/2f32),
        speed: 3.0,
        radius:40.0,
        color: Color::GREEN
        };
        
        let mut dash:Dash = Dash{visible: true,i:5};

let mut value:i32 = get_random_value(-100,100 );
let mut frame_count = 0;

    while !rl.window_should_close() {
        frame_count +=1;
        
        if frame_count % 60 ==0{
            value = get_random_value(-100, 100);
            frame_count = 0;
        }
        

        if rl.is_key_down(KEY_RIGHT){ball.position.x += ball.speed;}
        if rl.is_key_down(KEY_LEFT){ball.position.x -= ball.speed;}
        if rl.is_key_down(KEY_UP){ball.position.y -= ball.speed;}
        if rl.is_key_down(KEY_DOWN){ball.position.y += ball.speed;}

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON){
            ball.color=Color::BLACK;
            ball.position = ball.position.lerp(rl.get_mouse_position(),0.0025);
        }


        //Centre the ball slowly
        ball.position = ball.position.lerp(Vector2 { x: (SCREEN_WIDTH/2.0f32), y: (SCREEN_HEIGHT/2.0f32) }, 0.025f32);
        
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        dash.draw(&mut d);    
        draw_text_center(&mut d, "every 60 frames a new random value:", (SCREEN_HEIGHT as i32) /2 -40,20,Color::DARKPURPLE);
        draw_text_center(&mut d, format!("{}",value ).as_str(), (SCREEN_HEIGHT as i32) / 2,30,Color::BLUE);
        ball.draw(&mut d);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_text("Hello, world!", 12, 62, 30, Color::GREEN);

        d.draw_circle_v(ball.position, ball.radius, ball.color);
    }
}

fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32,font_size:i32, color:Color){
    let text_length = measure_text(text, font_size);
    d.draw_text(text, (SCREEN_WIDTH as i32) /2 - (text_length/2), y,font_size,color);
    d.draw_text("hello", 0, 50, 10, Color::GRAY);


}