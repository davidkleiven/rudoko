use super::board;
use std::env;
use log::{warn, info};


fn next_unknown_square(current: usize, board: &board::Board) -> usize {
    for idx in current+1..81 {
        if !board.index_is_known(idx) {
            return idx
        }
    }
    return 81;
}

fn prev_unknown_square(current: usize, board: &board::Board) -> usize{
    for idx in (0..current).rev() {
        if !board.index_is_known(idx) {
            return idx
        }
    }
    return 0
}

fn next_valid_number(idx: usize, board: &board::Board) -> u32 {
    let (r, c) = board::index2crd(idx);
    for num in board.get_index(idx)+1..10 {
        if board.valid_insert(r, c, num) {
            return num
        }
    }
    return 0;
}


fn brute_force_solve(board: &mut board::Board, max_attempts: i32) {
    let mut attempts = 0;
    let mut idx = 0;
    loop {
        let num = next_valid_number(idx, &board); // 0 if there are no valid
        board.set_index(idx, num);

        if num != 0 {
            idx = next_unknown_square(idx, &board);
        } else {
            idx = prev_unknown_square(idx, &board);
        }

        // If next index is outside the board, we have found a solution
        if idx == 81 {
            info!("Found solution after {} attempts", attempts);
            return;
        }

        attempts += 1;
        if max_attempts != -1 && attempts >= max_attempts {
            break;
        }
    }

    warn!("Did not find a solution within the maximum number of attempts...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_prev_unknown_square() {
        let mut board = board::empty_board();
        board.set_known(0, 1);
        
        let idx = next_unknown_square(0, &board);
        assert_eq!(idx, 2);

        let idx2 = prev_unknown_square(3, &board);
        assert_eq!(idx2, 2);

        let idx3 = prev_unknown_square(2, &board);
        assert_eq!(idx3, 0);
    }

    #[test]
    fn test_known_solutions() {
        let manifest_dir;
        match env::var("CARGO_MANIFEST_DIR") {
            Ok(val) => manifest_dir = val,
            Err(_e) => manifest_dir = "unknown".to_string()
        }

        assert_ne!(manifest_dir, "unknown");

        let sudokus = vec![
            ("simple1.txt", "simple1_solution.txt"),
            ("simple2.txt", "simple2_solution.txt"),
            ("simple3.txt", "simple3_solution.txt"),
            ("simple4.txt", "simple4_solution.txt")
        ];

        for (i, case) in sudokus.iter().enumerate() {
            let fname = format!("{}{}{}", manifest_dir, "/resources/", case.0);
            let mut board = board::load_board(fname);
        
            let max_attemps = 10000;
            brute_force_solve(&mut board, max_attemps);

            let solution_file = format!("{}{}{}", manifest_dir, "/resources/", case.1);
            let solution = board::load_board(solution_file);

            assert_eq!(board::is_equal(&solution, &board), true, "Test #{}", i);
        }
    }
}