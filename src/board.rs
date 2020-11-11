use crate::queen::Queen;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::fmt;

pub struct Board {
    pub size: usize,
    queens: Vec<Queen>,
    content: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            queens: Vec::with_capacity(size),
            size,
            content: (0..size).map(|_| vec![0; size]).collect(),
        }
    }

    pub fn init_queens(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.size {
            let q = Queen {
                x: i,
                y: rng.gen_range(0, self.size),
            };

            self.set(q.x, q.y);

            self.queens.push(q);
        }
    }

    pub fn is_taken(&self, x: usize, y: usize) -> bool {
        self.content[y][x] == 1
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.content[y][x] = 1;
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        self.content[y][x] = 0;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .content
            .clone()
            .into_iter()
            .fold(String::new(), |acc, row| {
                let line = row
                    .iter()
                    .map(|v| match v {
                        1 => "*",
                        _ => "_",
                    })
                    .collect::<Vec<&str>>()
                    .join(" ");
                acc + &line + "\n"
            });
        write!(f, "{}", output)
    }
}
