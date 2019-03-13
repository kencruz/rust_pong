use sdl2::rect::Rect;

pub enum Position {
    Left,
    Right,
}

pub struct Paddle {
    position: Position,
    y: i32,
    pub shape: Rect,
}

impl Paddle {
    pub fn new(pos: Position) -> Paddle {
        let shape = match &pos {
            Position::Left => Rect::new(50, 225, 20, 150),
            Position::Right => Rect::new(730, 225, 20, 150),
        };

        Paddle {
            position: pos,
            y: 50,
            shape: shape,
        }
    }
}
