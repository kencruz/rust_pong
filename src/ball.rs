use sdl2::rect::{Rect, Point};

pub struct Coords {
    x: i32,
    y: i32,
}

pub struct Velocity {
    x: i32,
    y: i32,
}

pub struct Ball {
    pub shape: Rect,
    pos: Coords,
    vel: Velocity,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            shape: Rect::from_center(Point::new(400, 300), 10, 10),
            pos: Coords{x: 400, y: 300},
            vel: Velocity{ x: 5, y: 0},
        }
    }

    pub fn update(&mut self) {
        if self.shape.center().x() < 0 {
            self.vel.x *= -1;
            self.shape.set_x(1);
        } else if self.shape.center().x() > 800 {
            self.vel.x *= -1;
            self.shape.set_x(795);
        } else {
            self.shape.set_x(self.shape.left() + self.vel.x);
            self.shape.set_y(self.shape.top() + self.vel.y);
        }
    }
}

