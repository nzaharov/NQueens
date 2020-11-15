use rand::Rng;
use std::env;
use std::io;
use std::time::Instant;

fn get_min_conflicts(row: &Vec<usize>) -> Vec<usize> {
    let min = row.iter().min().unwrap();
    row.iter()
        .cloned()
        .enumerate()
        .filter(|(_, c)| c == min)
        .map(|(i, _)| i)
        .collect()
}

// fn get_min_conflict(row: &Vec<usize>) -> usize {
//     row.iter()
//         .enumerate()
//         .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
//         .unwrap()
//         .0
// }

fn get_max_conflicts(queens: &Vec<usize>, conflicts: &Vec<Vec<usize>>) -> Vec<usize> {
    let max = queens
        .iter()
        .enumerate()
        .map(|(row, &col)| (row, conflicts[row][col]))
        .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
        .unwrap()
        .1;

    queens
        .iter()
        .enumerate()
        .map(|(row, &col)| (row, conflicts[row][col]))
        .filter(|(_, val)| *val == max)
        .map(|(row, _)| row)
        .collect()
}

// fn get_max_conflict(queens: &Vec<usize>, conflicts: &Vec<Vec<usize>>) -> usize {
//     queens
//         .iter()
//         .enumerate()
//         .map(|(row, &col)| (row, conflicts[row][col]))
//         .max_by(|(_, val1), (_, val2)| val1.cmp(val2))
//         .unwrap()
//         .0
// }

fn update_conflicts(
    row_index: usize,
    col_index: usize,
    conflicts: &mut Vec<Vec<usize>>,
    increment: bool,
) {
    let size = conflicts.len();
    for (index, row) in conflicts.iter_mut().enumerate() {
        if index != row_index {
            if increment {
                row[col_index] += 1;
            } else {
                row[col_index] = match row[col_index] > 0 {
                    true => row[col_index] - 1,
                    false => 0,
                };
            }
            let y = (row_index as i64 - index as i64).abs();
            let x1 = col_index as i64 - y;
            let x2 = col_index as i64 + y;
            if x1 >= 0 && x1 < size as i64 {
                if increment {
                    row[x1 as usize] += 1;
                } else {
                    row[x1 as usize] = match row[x1 as usize] > 0 {
                        true => row[x1 as usize] - 1,
                        false => 0,
                    };
                }
            }
            if x2 >= 0 && x2 < size as i64 {
                if increment {
                    row[x2 as usize] += 1;
                } else {
                    row[x2 as usize] = match row[x2 as usize] > 0 {
                        true => row[x2 as usize] - 1,
                        false => 0,
                    };
                }
            }
        }
    }
}

fn conflicts_exist(queens: &Vec<usize>, conflicts: &Vec<Vec<usize>>) -> bool {
    queens
        .iter()
        .enumerate()
        .any(|(row, &col)| conflicts[row][col] != 0)
}

fn main() -> io::Result<()> {
    let mut args = env::args_os();
    let verbose = args.any(|arg| arg == "-v" || arg == "--verbose");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut rng = rand::thread_rng();

    let size = buffer.trim_end().parse::<usize>().expect("Invalid input");

    let now = Instant::now();

    let mut queens: Vec<usize>;
    let mut conflicts: Vec<Vec<usize>>;
    let mut solved = false;
    const K: usize = 1;

    while {
        queens = vec![0; size];
        conflicts = vec![vec![0_usize; size]; size];
        for row in 0..size {
            let min_conflicts = get_min_conflicts(&conflicts[row]);
            let col = min_conflicts[rng.gen_range(0, min_conflicts.len())];
            queens[row] = col;
            update_conflicts(row, col, &mut conflicts, true);
        }
        println!("Board created ms: {}", now.elapsed().as_millis());
        let mut move_counter = 0;
        for _ in 0..K * size {
            if !conflicts_exist(&queens, &conflicts) {
                solved = true;
                break;
            }
            let max_conflicts = get_max_conflicts(&queens, &conflicts);
            let row = max_conflicts[rng.gen_range(0, max_conflicts.len())];
            // let row = get_max_conflict(&queens, &conflicts);
            let min_conflicts = get_min_conflicts(&conflicts[row]);
            let col = min_conflicts[rng.gen_range(0, min_conflicts.len())];
            // let rand_index = get_min_conflict(&conflicts[row]);
            update_conflicts(row, queens[row], &mut conflicts, false);
            queens[row] = col;
            update_conflicts(row, col, &mut conflicts, true);

            // Move counter
            move_counter += 1;
        }
        println!("Moves: {}", move_counter);
        !solved
    } {}

    if verbose {
        let board = queens
            .iter()
            .map(|&col| {
                let mut row = vec!["_"; queens.len()];
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
