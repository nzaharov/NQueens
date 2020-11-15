mod board;

use board::Board;
use std::env;
use std::io;
use std::time::Instant;

const K: usize = 1;

fn main() -> io::Result<()> {
    let mut args = env::args_os();
    let verbose = args.any(|arg| arg == "-v" || arg == "--verbose");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let size = buffer.trim_end().parse::<usize>().expect("Invalid input");

    // Relevant part
    let mut board: Board;
    let mut rng = rand::thread_rng();
    let mut solved = false;
    let now = Instant::now();
    while {
        board = Board::new(size);
        board.init(&mut rng);

        println!("Board created ms: {}", now.elapsed().as_millis());
        let mut move_counter = 0;
        for _ in 0..K * size {
            // let row_search = Instant::now();
            if !board.conflict_exists() {
                solved = true;
                break;
            }
            // println!("{}", row_search.elapsed().as_millis());
            let row = board.get_most_conflicted_row(&mut rng);
            let col = board.get_least_conflicted_col(row, &mut rng);
            board.move_to(row, col);

            // Increment moves
            move_counter += 1;
        }
        println!("Moves: {}", move_counter);
        !solved
    } {}

    if verbose {
        let board = board
            .queens
            .iter()
            .map(|&col| {
                let mut row = vec!["_"; board.size];
                row[col] = "*";
                row.join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", board);
    }
    println!("Time ms: {}", now.elapsed().as_millis());

    Ok(())
}
