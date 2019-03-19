use sdl2::rect::{Rect, Point};

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct Coordinates {
    x: f32,
    y: f32,
}

pub struct Ball {
    pub shape: Rect,
    pub vel: Velocity,
    pub coord: Coordinates,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            shape: Rect::from_center(Point::new(400, 300), 10, 10),
            vel: Velocity{ x: 5.0, y: 0.0},
            coord: Coordinates { x: 400.0, y: 300.0},
        }
    }

    pub fn update(&mut self) -> u32 {
        if self.shape.center().x() < 0 {
            self.vel.x *= -1.0;
            self.shape.set_x(1);
            self.coord.x = 1.0;
            return 1;
        } else if self.shape.center().x() > 800 {
            self.vel.x *= -1.0;
            self.shape.set_x(795);
            self.coord.x = 795.0;
            return 2;
        } else if self.shape.center().y() < 0 {
            self.vel.y *= -1.0;
            self.shape.set_y(1);
            self.coord.y = 1.0;
        } else if self.shape.center().y() > 600 {
            self.vel.y *= -1.0;
            self.shape.set_y(595);
            self.coord.y = 595.0;
        } else {
            self.coord.x += self.vel.x;
            self.coord.y += self.vel.y;
            self.shape.set_x(self.coord.x.floor() as i32);
            self.shape.set_y(self.coord.y.floor() as i32);
        }
        return 0;
    }
}

