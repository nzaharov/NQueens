use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt;

pub struct Board {
    pub size: usize,
    pub queens: Vec<usize>,
    cols: Vec<usize>,
    diag1: Vec<usize>,
    diag2: Vec<usize>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            queens: vec![0; size],
            cols: vec![0; size],
            diag1: vec![0; 2 * size - 1],
            diag2: vec![0; 2 * size - 1],
        }
    }

    pub fn init(&mut self, rng: &mut ThreadRng) {
        for row in 0..self.size {
            let col = self.get_least_conflicted_col(row, rng);
            self.queens[row] = col;
            self.increment_conflicts(row, col);
        }
    }

    pub fn conflict_exists(&self) -> bool {
        self.queens
            .iter()
            .enumerate()
            .map(|(row, &col)| {
                let conflicts =
                    self.cols[col] + self.diag1[col + row] + self.diag2[self.size - 1 + col - row];
                conflicts
            })
            .any(|conflict| conflict > 3)
    }

    pub fn get_most_conflicted_row(&self, rng: &mut ThreadRng) -> usize {
        let rows = self.queens.iter().enumerate().map(|(row, &col)| {
            let conflicts =
                self.cols[col] + self.diag1[col + row] + self.diag2[self.size - 1 + col - row];
            (row, conflicts)
        });
        let max = rows
            .clone()
            .max_by(|(_, conflicts1), (_, conflicts2)| conflicts1.cmp(conflicts2))
            .unwrap()
            .1;

        let max_conflict_rows = rows
            .filter(|(_, conflicts)| *conflicts == max)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        max_conflict_rows[rng.gen_range(0, max_conflict_rows.len())]
    }

    pub fn get_least_conflicted_col(&self, row_index: usize, rng: &mut ThreadRng) -> usize {
        let cols = (0..self.size).map(|i| {
            let conflicts = self.cols[i]
                + self.diag1[i + row_index]
                + self.diag2[self.size - 1 + i - row_index];
            (i, conflicts)
        });
        let min = cols
            .clone()
            .min_by(|(_, conflicts1), (_, conflicts2)| conflicts1.cmp(conflicts2))
            .unwrap()
            .1;

        let min_conflict_cols = cols
            .filter(|(_, conflicts)| *conflicts == min)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        min_conflict_cols[rng.gen_range(0, min_conflict_cols.len())]
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        self.decrement_conflicts(row, self.queens[row]);
        self.queens[row] = col;
        self.increment_conflicts(row, col);
    }

    fn decrement_conflicts(&mut self, row: usize, col: usize) {
        self.cols[col] -= 1;
        self.diag1[col + row] -= 1;
        self.diag2[self.size - 1 + col - row] -= 1;
    }

    fn increment_conflicts(&mut self, row: usize, col: usize) {
        self.cols[col] += 1;
        self.diag1[col + row] += 1;
        self.diag2[self.size - 1 + col - row] += 1;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let board = self
            .queens
            .iter()
            .map(|&col| {
                let mut row = vec!["_"; self.size];
                row[col] = "*";
                row.join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", board)
    }
}
