pub struct Queen {
    pub x: usize,
    pub y: usize,
}

impl Queen {
    fn is_safe(&self) -> bool {
        unimplemented!()
    }

    fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
