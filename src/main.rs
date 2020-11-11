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

    // let mut board: Vec<Vec<u8>> = (0..n).map(|_| vec![0; n]).collect();
    let mut board = Board::new(n);
    let mut queens: Vec<Queen> = Vec::with_capacity(n);

    for i in 0..n {
        // let mut q: Queen;

        // while {
        //     q = Queen {
        //         x: rng.gen_range(0, n),
        //         y: rng.gen_range(0, n),
        //     };
        //     board.is_taken(q.x, q.y)
        // } {}
        let q = Queen {
            x: i,
            y: rng.gen_range(0, n),
        };

        board.set(q.x, q.y);

        queens.push(q);
    }

    // while true {}

    // let output = board.into_iter().fold(String::new(), |acc, row| {
    //     let line = row
    //         .iter()
    //         .map(|v| match v {
    //             1 => "*",
    //             _ => "_",
    //         })
    //         .collect::<Vec<&str>>()
    //         .join(" ");
    //     acc + &line + "\n"
    // });

    println!("{}", board);

    Ok(())
}
