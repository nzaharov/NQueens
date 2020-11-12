use crate::queen::Queen;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::fmt;

pub struct Board {
    pub size: usize,
    queens: Vec<Queen>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            queens: Vec::with_capacity(size),
            size,
        }
    }

    pub fn init_queens(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.size {
            let q = Queen {
                x: i,
                y: rng.gen_range(0, self.size),
            };

            self.queens.push(q);
        }
    }

    pub fn conflict_exists(&self) -> bool {
        self.queens.iter().any(|q| self.is_contested(&q))
    }

    fn is_contested(&self, queen: &Queen) -> bool {
        self.queens.iter().any(|q| q.in_conflict_with(queen))
    }

    pub fn get_possible_positions(&self, queen: &Queen) -> Vec<(usize, usize)> {
        (0..self.size)
            .filter(|&i| i != queen.y)
            .map(|i| (queen.x, i))
            .collect()
    }

    pub fn get(&self, index: usize) -> &Queen {
        self.queens.get(index).unwrap()
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Queen {
        self.queens.get_mut(index).unwrap()
    }

    pub fn queen_conflicts(&self, queen: &Queen) -> usize {
        self.queens
            .iter()
            .filter(|q| queen.in_conflict_with(q))
            .count()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board: Vec<Vec<&str>> = (0..self.size).map(|_| vec!["_"; self.size]).collect();
        for q in self.queens.iter() {
            board[q.x][q.y] = "*";
        }
        let output = board.into_iter().fold(String::new(), |acc, row| {
            let line = row.join(" ");
            acc + &line + "\n"
        });
        write!(f, "{}", output)
    }
}
