use std::collections::HashSet;

pub fn valid_board(board: &Vec<Vec<char>>) -> bool {
    valid_rows_cols(board) && valid_squares(board)
}

fn valid_rows_cols(board: &Vec<Vec<char>>) -> bool {
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

fn valid_squares(board: &Vec<Vec<char>>) -> bool {
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

fn valid_list(list: &Vec<char>) -> bool {
    let mut seen = HashSet::new();

    for &c in list {
        if c != '0' && seen.contains(&c) { return false; }
        seen.insert(c);
    }
    
    true
}