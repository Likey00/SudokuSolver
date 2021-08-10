use std::io;

pub fn read_board() -> Option<Vec<Vec<char>>> {
    let mut board = vec![vec!['0'; 9]; 9];
    
    for i in 0..9 {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(10) => {
                input = input.trim().to_string();
                if !input.chars().all(char::is_numeric) { 
                    println!();
                    println!("Please enter only numeric characters");
                    return None; 
                }
                
                for j in 0..9 {
                    board[i][j] = input.chars().nth(j).unwrap();
                }
            }

            Ok(n) => {
                println!();
                println!("You entered {} characters on line {}, but we need 9", n-1, i+1);
                return None;
            }
            
            Err(_) => {
                println!("Error reading string!");
                return None;
            }
        }
    }

    Some(board)
}

pub fn print_board(board: &Vec<Vec<char>>) {
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