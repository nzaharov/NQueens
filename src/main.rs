mod board;
pub mod queen;

use board::Board;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut rng = rand::thread_rng();

    let n = buffer.trim_end().parse::<usize>().expect("Invalid input ");

    let mut board = Board::new(n);
    board.init_queens(&mut rng);
    // while true {}

    println!("{}", board);

    Ok(())
}
