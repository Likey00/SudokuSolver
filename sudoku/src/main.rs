mod board_utils;
mod board_checker;
mod board_solver;

fn main() {
    println!("Please enter your sudoku board in 9 lines, 9 characters per line.");
    println!("Numbers which are known should be input where they are, and empty spaces should be denoted by a 0.");

    let mut board = match board_utils::read_board() {
        Ok(board) => board,
        Err(msg) => panic!("{}", msg),
    };

    println!();
    
    if board_solver::solve(&mut board) { 
        board_utils::print_board(&board)
    }
    else { println!("This board has no solution!"); }
}