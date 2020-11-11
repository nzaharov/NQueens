#[derive(PartialEq)]
pub struct Queen {
    pub x: usize,
    pub y: usize,
}

impl Queen {
    fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
