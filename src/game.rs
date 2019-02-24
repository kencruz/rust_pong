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
        let texture_creator = canvas.texture_creator();
        let mut events = sdl_context.event_pump().unwrap();

        let mut player = Rect::new(50, 225, 20, 150);
        let cpu = Rect::new(730, 225, 20, 150);
        let score = Rect::from_center(Point::new(400, 30), 100, 40);

        let path: &Path = Path::new("../droid.ttf");
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let font = ttf_context.load_font(path, 18).unwrap();

        let surface = font
            .render("0 - 0")
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

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
            if keys.contains(&Keycode::Up) {
                player.set_y(player.top() - 5);
            }

            if keys.contains(&Keycode::Down) {
                player.set_y(player.top() + 5);
            }

            // render
            // background
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            // rectangle
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(player).unwrap();
            canvas.fill_rect(cpu).unwrap();
            //score
            canvas.copy(&texture, None, score).unwrap();

            canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}