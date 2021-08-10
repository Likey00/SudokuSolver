use crate::board_checker;
use crate::board_utils::Board;

/// Takes in a board, and returns true if the board was
/// solved, and false otherwise
pub fn solve(board: &mut Board) -> bool {
    solve_helper(board, 0, 0)
}

/// Takes in the board, current row, current col, and returns true 
/// if board was solved, and false if it couldn't be solved
fn solve_helper(board: &mut Board, row: usize, col: usize) -> bool {
    if row == 9 { return true; }
    
    let (next_row, next_col) = match col {
        8 => (row+1, 0),
        _ => (row, col+1),
    };

    if board[row][col] != '0' { return solve_helper(board, next_row, next_col); }

    for g in '1'..='9' {
        board[row][col] = g;
        if board_checker::valid_board(&board) && solve_helper(board, next_row, next_col) { 
            return true;
        }
    }
    
    board[row][col] = '0';
    false
}