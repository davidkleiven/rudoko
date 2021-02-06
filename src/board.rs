use std::env;
use std::io::BufReader;
use std::io::BufRead;

pub struct Board {
    numbers: [u32; 81],
    known: [bool; 81]
}

fn index(row: usize, col: usize) -> usize {
    return row*9 + col
}
impl Board {
    fn get(&self, row: usize, col: usize) -> u32 {
        return self.numbers[index(row, col)]
    }

    fn set(&mut self, row: usize, col: usize, value: u32) {
        self.numbers[index(row, col)] = value
    }

    fn is_known(&self, row: usize, col: usize) -> bool {
        return self.known[index(row, col)]
    }

    fn set_known(&mut self, row: usize, col: usize) {
        self.known[index(row, col)] = true
    }
}

fn empty_board() -> Board {
    return Board{
        numbers: [0; 81],
        known: [false; 81]
    }
}

/// Returns a board loaded from a comma separated file
///
/// # Arguments
/// 
/// * `fname` - Filename with the sudoku board
/// 
/// The sudoku board should be listed in a comma separated file where
/// known entries are given by their numbers and unknown fields are indicated with
/// x
/// 
/// Example (showing the two first rows)
/// 
/// 3,x,x,x,x,4,x,8,x
/// x,2,x,x,x,1,x,7,x
pub fn load_board(fname: String) -> Board {
    let mut board = empty_board();

    let f = match std::fs::File::open(fname) {
        Ok(value) => value,
        Err(e) => {
            panic!("Could not open CSV file".to_string() + &e.to_string())
        }
    };

    let file = BufReader::new(f);

    for (row, line) in file.lines().enumerate() {
        for (col, value) in line.unwrap().split(",").enumerate() {
            if value != "x" {
                let num: u32 = value.parse().unwrap();
                board.set_known(row, col);
                board.set(row, col, num);
            }
        }
    }
    return board
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut board = empty_board();
        board.set(3, 4, 5);
        assert_eq!(board.get(3, 4), 5);
    }

    #[test]
    fn test_set_get_known() {
        let mut board = empty_board();
        assert_eq!(board.is_known(4, 5), false);
        board.set_known(4, 5);
        assert_eq!(board.is_known(4, 5), true);
    }

    // SparseBoard holds row, column and expected value
    struct SparseBoard {
        row: usize,
        col: usize,
        value: u32
    }

    #[test]
    fn test_read_board() {
        let manifest_dir;
        match env::var("CARGO_MANIFEST_DIR") {
            Ok(val) => manifest_dir = val,
            Err(_e) => manifest_dir = "unknown".to_string()
        }

        assert_ne!(manifest_dir, "unknown");

        let fname = manifest_dir + "/resources/board1.csv";
        let board = load_board(fname);

        let expect = vec![
            SparseBoard{row: 0, col: 0, value: 1},
            SparseBoard{row: 0, col: 3, value: 2},
            SparseBoard{row: 0, col: 6, value: 3},
            SparseBoard{row: 1, col: 0, value: 4},
            SparseBoard{row: 1, col: 3, value: 5},
            SparseBoard{row: 1, col: 6, value: 6},
            SparseBoard{row: 2, col: 0, value: 7},
            SparseBoard{row: 2, col: 3, value: 8},
            SparseBoard{row: 2, col: 6, value: 9},

            SparseBoard{row: 3, col: 1, value: 1},
            SparseBoard{row: 3, col: 4, value: 2},
            SparseBoard{row: 3, col: 7, value: 3},
            SparseBoard{row: 4, col: 1, value: 4},
            SparseBoard{row: 4, col: 4, value: 5},
            SparseBoard{row: 4, col: 7, value: 6},
            SparseBoard{row: 5, col: 1, value: 7},
            SparseBoard{row: 5, col: 4, value: 8},
            SparseBoard{row: 5, col: 7, value: 9},

            SparseBoard{row: 6, col: 2, value: 1},
            SparseBoard{row: 6, col: 5, value: 2},
            SparseBoard{row: 6, col: 8, value: 3},
            SparseBoard{row: 7, col: 2, value: 4},
            SparseBoard{row: 7, col: 5, value: 5},
            SparseBoard{row: 7, col: 8, value: 6},
            SparseBoard{row: 8, col: 2, value: 7},
            SparseBoard{row: 8, col: 5, value: 8},
            SparseBoard{row: 8, col: 8, value: 9}
        ];

        for sp in expect {
            assert_eq!(board.get(sp.row, sp.col), sp.value);
            assert_eq!(board.is_known(sp.row, sp.col), true)
        }
    }
}