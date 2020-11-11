mod board;
mod queen;

use board::Board;
use queen::Queen;
use rand::Rng;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut rng = rand::thread_rng();

    let n = buffer.trim_end().parse::<usize>().expect("Invalid input ");

    let mut board = Board::new(n);
    let mut queens: Vec<Queen> = Vec::with_capacity(n);

    for i in 0..n {
        let q = Queen {
            x: i,
            y: rng.gen_range(0, n),
        };

        board.set(q.x, q.y);

        queens.push(q);
    }

    println!("{}", board);

    Ok(())
}
