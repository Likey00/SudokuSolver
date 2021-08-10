use std::collections::HashSet;
use crate::board_utils::Board;

/// Returns true if all rows, columns, and the 9
/// 3x3 sudoku squares have no nonzero repeats within 
/// them, false otherwise
pub fn valid_board(board: &Board) -> bool {
    valid_rows_cols(board) && valid_squares(board)
}

/// Returns true if all rows and columns have no nonzero
/// repeats, false otherwise
fn valid_rows_cols(board: &Board) -> bool {
    for i in 0..9 {
        let (mut horiz, mut vert) = (Vec::new(), Vec::new());
        
        for j in 0..9 {
            horiz.push(board[i][j]);
            vert.push(board[j][i]);
        }
        
        if !valid_list(&horiz) || !valid_list(&vert) { return false; }
    }
    
    true
}

/// Returns true if 3x3 sudoku squares have no nonzero
/// repeats, false otherwise
fn valid_squares(board: &Board) -> bool {
    for row in (0..9).step_by(3) {
        for col in (0..9).step_by(3) {
            let mut square = Vec::new();

            for i in 0..3 {
                for j in 0..3 { square.push(board[row+i][col+j]); }
            }

            if !valid_list(&square) { return false; }
        }
    }

    true
}

/// Returns true if the given vector has no repeats,
/// false otherwise
fn valid_list(list: &Vec<char>) -> bool {
    let mut seen = HashSet::new();

    for &c in list {
        if c != '0' && seen.contains(&c) { return false; }
        seen.insert(c);
    }
    
    true
}