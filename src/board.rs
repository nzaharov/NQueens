use rand::rngs::ThreadRng;
use rand::Rng;

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
        for i in 0..self.size {
            let mut conflicts = 0;
            for j in 0..self.size {
                if self.queens[j] == i {
                    conflicts += 1;
                }
            }
            if conflicts > 1 {
                return true;
            }
        }

        for i in 0..2 * self.size - 1 {
            let mut conflicts = 0;
            for j in 0..self.size {
                if self.queens[j] + j == i {
                    conflicts += 1;
                }
            }
            if conflicts > 1 {
                return true;
            }
        }

        let n = self.size as i64;
        for i in (1 - n)..n {
            let mut conflicts = 0;
            for j in 0..self.size {
                if self.queens[j] as i64 - j as i64 == i {
                    conflicts += 1;
                }
            }
            if conflicts > 1 {
                return true;
            }
        }

        false
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
