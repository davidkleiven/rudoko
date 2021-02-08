mod board;
mod brute_force_solver;
use clap::{Arg, App};
use std::path::Path;

fn main() {
    let matches = App::new("rudoku")
        .version("0.1")
        .author("D. Kleiven")
        .about("Solves sudoku's using a brute force search. The sudoku is given as via a text file where  \
each line corresponds to a row. Unknown values are represented with the x character.
Example:
xxxxx7xxx
x65xxx89x
4xx5x1xx7
89x1x5x2x
xx26x43xx
x4x9x3x1x
3xx7x8xx4
x57xxx98x
xxxx3xxxx
")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("Text file containing the sudoku")
            .takes_value(true))
        .arg(Arg::with_name("nmax")
            .short("n")
            .long("nmax")
            .help("Maximum number of steps used in the search. If -1 no limit is imposed")
            .takes_value(true)
            .default_value("-1"))
        .get_matches();

    let input_file;
    match matches.value_of("input") {
        Some(inp) => input_file = inp.to_string(),
        None => input_file = "unknown".to_string(),
    }

    if input_file == "unknown" {
        print!("No input file provided\n");
        return;
    }

    if !Path::new(&input_file).exists() {
        println!("Cannot find file {}\n", input_file);
        return;
    }

    let nmax;
    match matches.value_of("nmax").unwrap().parse::<i32>() {
        Ok(p) => nmax = p,
        Err(_e) => nmax = -1,
    }

    if nmax == -1 {
        print!("Solving sudouko using an unlimited number of steps\n");
    } else {
        println!("Solving sudoku using maximum {} steps\n", nmax);
    }

    let mut board = board::load_board(input_file);
    print!("Initial soduku (0 represent unknown values):\n");
    board.print();
    print!("\n");
    brute_force_solver::brute_force_solve(&mut board, nmax);
    print!("Solution:\n");
    board.print();
}
