use crate::ball::Ball;
use crate::paddle::{Paddle, Position};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

pub struct Game {}

impl Game {
    pub fn run(&self) {
        // initialize
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("pong demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let mut events = sdl_context.event_pump().unwrap();

        let mut player = Paddle::new(Position::Left);
        let mut cpu = Paddle::new(Position::Right);
        let mut ball = Ball::new();

        // scoreboard initialization
        let texture_creator = canvas.texture_creator();
        let score = Rect::from_center(Point::new(400, 30), 100, 40);
        let path: &Path = Path::new("../droid.ttf");
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let font = ttf_context.load_font(path, 64).unwrap();
        let mut player_score = 0;
        let mut cpu_score = 0;

        'running: loop {
            // event
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            // Create a set of pressed Keys.
            let keys: HashSet<Keycode> = events
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect();

            if !keys.is_empty() {
                println!("keys: {:?}", keys);
            }

            // loop
            if keys.contains(&Keycode::Up) && player.shape.y() > 0 {
                player.shape.set_y(player.shape.top() - 5);
            }

            if keys.contains(&Keycode::Down)
                && player.shape.y() + (player.shape.height() as i32) < 600
            {
                player.shape.set_y(player.shape.top() + 5);
            }

            // CPU AI
            if ball.shape.y() < cpu.shape.y() && ball.shape.x() > 300 {
                cpu.shape.set_y(cpu.shape.top() - 3);
            }

            if ball.shape.y() > cpu.shape.y() + (cpu.shape.height() as i32) && ball.shape.x() > 300
            {
                cpu.shape.set_y(cpu.shape.top() + 3);
            }

            // change angle of ball when colliding with paddle

            if ball.shape.has_intersection(player.shape) && ball.vel.x < 0.0 {
                ball.vel.x *= -1.0;
                match ball.shape.y() - player.shape.y() {
                    x if x < 150 && x >= 125 => ball.vel.y = 2.0,
                    x if x < 125 && x >= 100 => ball.vel.y = 1.0,
                    x if x < 100 && x >= 75 => ball.vel.y = 0.2,
                    x if x < 75 && x >= 50 => ball.vel.y = -0.2,
                    x if x < 50 && x >= 25 => ball.vel.y = -1.0,
                    _ => ball.vel.y = -2.0,
                }
                println!("Velocity is {}", ball.vel.y);
            }

            if ball.shape.has_intersection(cpu.shape) && ball.vel.x > 0.0 {
                ball.vel.x *= -1.0;
                match ball.shape.y() - cpu.shape.y() {
                    x if x < 150 && x >= 125 => ball.vel.y = 2.0,
                    x if x < 125 && x >= 100 => ball.vel.y = 1.0,
                    x if x < 100 && x >= 75 => ball.vel.y = 0.2,
                    x if x < 75 && x >= 50 => ball.vel.y = -0.2,
                    x if x < 50 && x >= 25 => ball.vel.y = -1.0,
                    _ => ball.vel.y = -2.0,
                }
                println!("Velocity is {}", ball.vel.y);
            }

            // scoreboard formatting and texture
            let surface = font
                .render(&format!("{} - {}", player_score, cpu_score))
                .blended(Color::RGBA(255, 255, 255, 255))
                .map_err(|e| e.to_string())
                .unwrap();

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            // update scoreboard if ball goes out of bounds
            match ball.update() {
                1 => cpu_score += 1,
                2 => player_score += 1,
                _ => {},
            }
            // render
            // background
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            // entities
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(player.shape).unwrap();
            canvas.fill_rect(cpu.shape).unwrap();
            canvas.fill_rect(ball.shape).unwrap();
            //score
            canvas.copy(&texture, None, score).unwrap();

            canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
