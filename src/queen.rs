#[derive(PartialEq, Debug)]
pub struct Queen {
    pub x: usize,
    pub y: usize,
}

impl Queen {
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn in_conflict_with(&self, q: &Queen) -> bool {
        self != q
            && ((self.x as f64 - q.x as f64).abs() == (self.y as f64 - q.y as f64).abs()
                || self.x == q.x
                || self.y == q.y)
    }
}
