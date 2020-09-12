#[derive(Debug)]
pub enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug)]
pub struct Options {
    shift: f32,
    direction: Direction,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            shift: 2.0,
            direction: Direction::BottomRight,
        }
    }
}

impl Options {
    pub fn char_shift(&self) -> f32 {
        match self.direction {
            Direction::TopLeft | Direction::Left | Direction::BottomLeft => -self.shift,
            Direction::Bottom | Direction::Top => 0.0,
            Direction::TopRight | Direction::Right | Direction::BottomRight => self.shift,
        }
    }

    pub fn line_shift(&self) -> f32 {
        match self.direction {
            Direction::TopLeft | Direction::Top | Direction::TopRight => -self.shift,
            Direction::Left | Direction::Right => 0.0,
            Direction::BottomLeft | Direction::Bottom | Direction::BottomRight => self.shift,
        }
    }

    pub fn set_shift(&mut self, shift: u8) {
        self.shift = shift as f32;
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.direction = dir;
    }
}
