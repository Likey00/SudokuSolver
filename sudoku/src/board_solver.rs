use crate::board_checker;

pub fn solve(board: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    for start in '1'..='9' {
        if let Some(v) = solve_helper(board.clone(), 0, 0, start) {
            return Some(v);
        }
    }
    return None
}

fn solve_helper(board: Vec<Vec<char>>, row: usize, col: usize, guess: char) -> Option<Vec<Vec<char>>> {
    if row == 9 { return Some(board) }
    
    let (next_row, next_col) = match col {
        8 => (row+1, 0),
        _ => (row, col+1),
    };

    if board[row][col] != '0' { return solve_helper(board, next_row, next_col, guess); }

    let mut sol = board.to_vec();
    sol[row][col] = guess;
    if !board_checker::valid_board(&sol) { return None; }

    for g in '1'..='9' {
        if let Some(v) = solve_helper(sol.clone(), next_row, next_col, g) {
            return Some(v);
        }
    }
    
    None
}