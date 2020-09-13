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
    seed: f32,
    shift: f32,
    direction: Direction,
    invert: bool,
    use_stdin: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            seed: rand::random::<f32>() * 360.,
            shift: 2.0,
            direction: Direction::BottomRight,
            invert: false,
            use_stdin: true,
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

    pub fn set_shift(&mut self, shift: i16) {
        self.shift = shift as f32;
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn seed(&self) -> f32 {
        self.seed
    }

    pub fn set_seed(&mut self, seed: u16) {
        self.seed = (seed % 360) as f32;
    }

    pub fn use_stdin(&self) -> bool {
        self.use_stdin
    }

    pub fn set_use_stdin(&mut self, use_stdin: bool) {
        self.use_stdin = use_stdin;
    }

    pub fn invert(&self) -> bool {
        self.invert
    }

    pub fn set_invert(&mut self, invert: bool) {
        self.invert = invert;
    }
}
