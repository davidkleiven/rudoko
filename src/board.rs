use std::env;
use std::io::BufReader;
use std::io::BufRead;

pub struct Board {
    numbers: [u32; 81],
    known: [bool; 81]
}

pub fn index(row: usize, col: usize) -> usize {
    return row*9 + col
}

pub fn index2crd(idx: usize) -> (usize, usize) {
    return (idx/9, idx % 9)
}

pub fn is_equal(b1: &Board, b2: &Board) -> bool {
    for idx in 0..81 {
        if b1.get_index(idx) != b2.get_index(idx) {
            return false
        }
    }
    return true
}

impl Board {
    pub fn get(&self, row: usize, col: usize) -> u32 {
        return self.get_index(index(row, col))
    }

    pub fn set(&mut self, row: usize, col: usize, value: u32) {
        self.set_index(index(row, col), value);
    }

    pub fn is_known(&self, row: usize, col: usize) -> bool {
        return self.index_is_known(index(row, col))
    }

    pub fn set_known(&mut self, row: usize, col: usize) {
        self.known[index(row, col)] = true
    }

    pub fn index_is_known(&self, idx: usize) -> bool {
        return self.known[idx]
    }

    pub fn get_index(&self, idx: usize) -> u32 {
        return self.numbers[idx]
    }

    pub fn set_index(&mut self, idx: usize, value: u32) {
        self.numbers[idx] = value
    }

    fn is_in_row(&self, row: usize, value: u32) -> bool {
        for c in 0..9 {
            if self.get(row, c) == value {
                return true
            }
        }
        return false
    }

    fn is_in_col(&self, col: usize, value: u32) -> bool {
        for r in 0..9 {
            if self.get(r, col) == value {
                return true
            }
        }
        return false
    }

    /// is_in_tile checks if the passed number is in one of the tiles given by
    /// given by trow and tcol. 0 < trow < 3 and 0 < tcol < 3
    fn is_in_tile(&self, trow: usize, tcol: usize, value: u32) -> bool {
        let row_start = trow*3;
        let col_start = tcol*3;
        for r in row_start..row_start+3 {
            for c in col_start..col_start+3 {
                if self.get(r, c) == value {
                    return true
                }
            }
        }
        return false
    }

    /// valid_insert returns true if the passed value can be inserted at the suggested position
    pub fn valid_insert(&self, row: usize, col: usize, value: u32) -> bool {
        return !self.is_in_row(row, value) && !self.is_in_col(col, value) && !self.is_in_tile(row/3, col/3, value)
    }

    pub fn print(&self) {
        for r in 0..9 {
            for c in 0..9 {
                println!("{}", self.get(r, c));
            }
            print!("");
        }
    }
}

pub fn empty_board() -> Board {
    return Board{
        numbers: [0; 81],
        known: [false; 81]
    }
}

/// Returns a board loaded from a text separated file
///
/// # Arguments
/// 
/// * `fname` - Filename with the sudoku board
/// 
/// The sudoku board should be listed in a text separated file where
/// known entries are given by their numbers and unknown fields are indicated with
/// x
/// 
/// Example:
/// 
/// xxxxx7xxx
/// x65xxx89x
/// 4xx5x1xx7
/// 89x1x5x26
/// xx26x43xx
/// x4x9x3x1x
/// 3xx7x8xx4
/// x57xxx98x
/// xxxx3xxxx
pub fn load_board(fname: String) -> Board {
    let mut board = empty_board();

    let f = match std::fs::File::open(fname) {
        Ok(value) => value,
        Err(e) => {
            panic!("Could not open txt file".to_string() + &e.to_string())
        }
    };

    let file = BufReader::new(f);

    for (row, line) in file.lines().enumerate() {
        for (col, value) in line.unwrap().chars().enumerate() {
            if value != 'x' && value.is_digit(10) {
                let num: u32 = value.to_digit(10).unwrap();
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

        let fname = manifest_dir + "/resources/board1.txt";
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

    #[test]
    fn test_in_row_col() {
        let mut board = empty_board();
        board.set(0, 1, 1);
        board.set(7, 3, 2);
        assert_eq!(board.is_in_row(0, 1), true);
        assert_eq!(board.is_in_row(7, 2), true);
        assert_eq!(board.is_in_row(3, 2), false);
        assert_eq!(board.is_in_row(1, 1), false);

        assert_eq!(board.is_in_col(0, 1), false);
        assert_eq!(board.is_in_col(7, 2), false);
        assert_eq!(board.is_in_col(3, 2), true);
        assert_eq!(board.is_in_col(1, 1), true);

        assert_eq!(board.is_in_tile(0, 0, 1), true);
        assert_eq!(board.is_in_tile(2, 1, 2), true);
        assert_eq!(board.is_in_tile(0, 0, 2), false);
        assert_eq!(board.is_in_tile(2, 1, 1), false);

        assert_eq!(board.valid_insert(0, 0, 1), false);
        assert_eq!(board.valid_insert(0, 0, 2), true);
        assert_eq!(board.valid_insert(8, 3, 2), false);
        assert_eq!(board.valid_insert(7, 4, 2), false);
        assert_eq!(board.valid_insert(8, 4, 2), false);
        assert_eq!(board.valid_insert(8, 5, 2), false);
    }

    #[test]
    fn test_idx2row() {
        for idx in 0..81 {
            let (r, c) = index2crd(idx);
            let got = index(r, c);
            assert_eq!(idx, got);
        }
    }
}