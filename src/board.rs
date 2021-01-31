
struct Board {
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
}