use std::io;

fn print_board(board: &[[char; 3]; 3]) {
    println!(" {} | {} | {}", board[0][0], board[0][1], board[0][2]);
    println!("-----------");
    println!(" {} | {} | {}", board[1][0], board[1][1], board[1][2]);
    println!("-----------");
    println!(" {} | {} | {}", board[2][0], board[2][1], board[2][2]);
}

fn check_winner(board: &[[char; 3]; 3]) -> Option<char> {
    // Check rows
    for row in board {
        if row[0] == row[1] && row[1] == row[2] && row[0] != ' ' {
            return Some(row[0]);
        }
    }

    // Check columns
    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] && board[0][col] != ' '
        {
            return Some(board[0][col]);
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ' {
        return Some(board[0][0]);
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != ' ' {
        return Some(board[0][2]);
    }

    None
}

fn player_turn(board: &mut [[char; 3]; 3]) {
    loop {
        println!("Enter the x coordinate (0-2):");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let x: usize = match input.trim().parse() {
            Ok(num) if num < 3 => num,
            _ => {
                println!("Invalid input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        println!("Enter the y coordinate (0-2):");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let y: usize = match input.trim().parse() {
            Ok(num) if num < 3 => num,
            _ => {
                println!("Invalid input. Please enter a number between 0 and 2.");
                continue;
            }
        };

        if board[y][x] == ' ' {
            board[y][x] = 'X';
            break;
        } else {
            println!("That space is already occupied. Try again.");
        }
    }
}

fn computer_turn(board: &mut [[char; 3]; 3]) {
    let mut best_score = i32::MIN;
    let mut best_move = (0, 0);

    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == ' ' {
                board[i][j] = 'O';
                let score = minimax(board, 0, false);
                board[i][j] = ' ';
                if score > best_score {
                    best_score = score;
                    best_move = (i, j);
                }
            }
        }
    }

    board[best_move.0][best_move.1] = 'O';
}

fn minimax(board: &mut [[char; 3]; 3], depth: i32, is_maximizing: bool) -> i32 {
    if let Some(winner) = check_winner(board) {
        return match winner {
            'O' => 10 - depth,
            'X' => depth - 10,
            _ => 0,
        };
    }

    if check_free_spaces(board) == 0 {
        return 0;
    }

    if is_maximizing {
        let mut best_score = i32::MIN;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == ' ' {
                    board[i][j] = 'O';
                    let score = minimax(board, depth + 1, false);
                    board[i][j] = ' ';
                    best_score = best_score.max(score);
                }
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == ' ' {
                    board[i][j] = 'X';
                    let score = minimax(board, depth + 1, true);
                    board[i][j] = ' ';
                    best_score = best_score.min(score);
                }
            }
        }
        best_score
    }
}

fn reset_board() -> [[char; 3]; 3] {
    [[' '; 3]; 3]
}

fn check_free_spaces(board: &[[char; 3]; 3]) -> i32 {
    board
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == ' ')
        .count() as i32
}

fn print_winner(winner: char) {
    if winner == 'X' {
        println!("Player wins!");
    } else if winner == 'O' {
        println!("Computer wins!");
    } else {
        println!("It's a draw!");
    }
}

fn main() {
    let mut board = reset_board();

    loop {
        print_board(&board);
        player_turn(&mut board);

        if let Some(winner) = check_winner(&board) {
            print_board(&board);
            print_winner(winner);
            break;
        }

        if check_free_spaces(&board) == 0 {
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        computer_turn(&mut board);

        if let Some(winner) = check_winner(&board) {
            print_board(&board);
            print_winner(winner);
            break;
        }

        if check_free_spaces(&board) == 0 {
            print_board(&board);
            println!("It's a draw!");
            break;
        }
    }
}
