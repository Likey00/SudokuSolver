use std::io;

pub type Board = Vec<Vec<char>>;

/// Reads in a board from input line by line and
/// returns it as a Board, or Err if
/// something went wrong
pub fn read_board() -> Result<Board, String> {
    let mut board = vec![vec!['0'; 9]; 9];
    
    for i in 0..9 {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(10) => {
                input = input.trim().to_string();
                if !input.chars().all(char::is_numeric) { 
                    return Err(String::from("Non-numeric characters in input"));
                }
                
                for j in 0..9 {
                    board[i][j] = input.chars().nth(j).unwrap();
                }
            }

            Ok(_) => {
                return Err(String::from("Incorrect number of characters"));
            }
            
            Err(_) => {
                return Err(String::from("Error reading string"));
            }
        }
    }

    Ok(board)
}

/// Nicely prints out a given board
pub fn print_board(board: &Board) {
    for i in 0..9 {
        print!(" ");
        for j in 0..9 {
            print!("{} ", board[i][j]);

            if j == 2 || j == 5 {
                print!("| ");
            }
        }

        println!();
        if i == 2 || i == 5 {
            println!("{}", "-".repeat(23));
        }
    }
}