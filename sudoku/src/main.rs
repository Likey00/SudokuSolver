mod board_utils;
mod board_checker;
mod board_solver;

fn main() {
    println!("Please enter your sudoku board in 9 lines, 9 characters per line.");
    println!("Numbers which are known should be input where they are, and empty spaces should be denoted by a 0.");

    let board = match board_utils::read_board() {
        Some(vec) => vec,
        None => {
            println!("Terminating due to invalid board");
            std::process::exit(1);
        }
    };

    println!();

    match board_solver::solve(board) {
        Some(v) => board_utils::print_board(&v),
        None => println!("This board has no solution!"),
    }
}