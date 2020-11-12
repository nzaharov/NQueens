mod board;
pub mod queen;

use board::Board;
use queen::Queen;
use rand::Rng;
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut rng = rand::thread_rng();

    let size = buffer.trim_end().parse::<usize>().expect("Invalid input ");

    let mut board = Board::new(size);
    board.init_queens(&mut rng);

    let now = Instant::now();
    while board.conflict_exists() {
        let index = rng.gen_range(0, size);
        let queen = board.get(index);

        let possibilities = board.get_possible_positions(&queen);
        let mut min = size + 1;
        let mut best_position: Option<(usize, usize)> = None;

        for (x, y) in possibilities {
            let temp_queen = Queen { x, y };

            let current_conflicts = board.queen_conflicts(&temp_queen);
            if current_conflicts < min {
                min = current_conflicts;
                best_position = Some((x, y));
            }
        }
        let queen = board.get_mut(index);
        let (x, y) = best_position.unwrap();
        queen.move_to(x, y);
    }

    println!("{}", board);
    println!("Time ms: {}",now.elapsed().as_millis());

    Ok(())
}
