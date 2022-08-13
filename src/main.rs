use std::time::SystemTime;
use std::fs;

const WIDTH: u8 = 9;

fn parse_board(file_path: &str) -> [[u8; WIDTH as usize]; WIDTH as usize] {
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => "Something went wrong reading the file".to_string(),
    };

    let mut board: [[u8; WIDTH as usize]; WIDTH as usize] = [[0; WIDTH as usize]; WIDTH as usize];
    let mut rows: u8 = 0;
    let mut cols: u8 = 0;

    for row in file_content.chars() {
        if cols == WIDTH {
            cols = 0;
            rows += 1;
        } 
        if row.is_numeric() {
            board[rows as usize][cols as usize] =
                row.to_digit(10).unwrap() as u8;
            cols += 1;
        }
    }

    return board;
}

fn is_safe(board: [[u8; WIDTH as usize]; WIDTH as usize], row: u8, col: u8, val: u8) -> bool {
    // check the rows
    for c in 0..WIDTH {
        if board[row as usize][c as usize] == val {
            return false;
        }
    }

    // check the cols
    for r in 0..WIDTH {
        if board[r as usize][col as usize] == val {
            return false;
        }
    }

    // check the inner square
    let inner_square_row_start: u8 = row - row % 3;
    let inner_square_col_start: u8 = col - col % 3;

    for r in 0..3 {
        for c in 0..3 {
            if board[(inner_square_row_start + r) as usize]
                    [(inner_square_col_start + c) as usize]
                    == val {
                return false
            }
        }
    }
    return true;
}

fn dfs(board: &mut [[u8; WIDTH as usize]; WIDTH as usize], row: u8, col: u8) -> bool {
    let mut row: u8 = row;
    let mut col: u8 = col;

    if row  == WIDTH - 1 && col == WIDTH {
        return true;
    }

    if col == WIDTH {
        col = 0;
        row += 1;
    }

    if board[row as usize][col as usize] > 0 {
        return dfs(board, row, col + 1);
    }

    for val in 1..(WIDTH + 1) {
        if is_safe(*board, row, col, val) {
            board[row as usize][col as usize] = val;

            if dfs(board, row, col + 1) {
                return true;
            }
        }
        board[row as usize][col as usize] = 0;
    }
    return false;
}

fn solve(board: &mut [[u8; WIDTH as usize]; WIDTH as usize]) -> Option<[[u8; WIDTH as usize]; WIDTH as usize]> {
    if dfs(board, 0, 0) {
        Some(*board)
    } else {
        None
    }
}

fn display_board(board: [[u8; WIDTH as usize]; WIDTH as usize]) {
    let mut board_row = 0;
    let mut board_col = 0;
    let divider = "-------------------------";

    for row in 0..13 {
        if row % 4 == 0 {
            println!("{}", divider.to_string());
            continue;
        }

        for col in 0..13 {
            if col % 4 == 0 {
                match col / 4 {
                    0 => print!("| "),
                    1 | 2 => print!(" | "),
                    _ => print!(" |")
                }
                continue;
            }

            if board_col % 3 == 1 {
                print!(" {} ", board[board_row][board_col]);
            } else {
                print!("{}", board[board_row][board_col]);
            }
            board_col += 1;

            if board_col == WIDTH as usize  {
                board_col = 0;
                board_row += 1
            }
        }
        println!();
    }
}

fn main() {
    let board: [[u8; WIDTH as usize]; WIDTH as usize] =
                    parse_board("/home/razvan/dev/rust/sudoku-solver/board.txt");

    let start_time = SystemTime::now();

    let mut solved_board: [[u8; WIDTH as usize]; WIDTH as usize] = board;
    match solve(&mut solved_board) {
        Some(board) => display_board(board),
        None => panic!("Could not solve the board"),
    }

    let time_taken = start_time.elapsed().unwrap().as_secs_f64();
    println!("Puzzle solved in {}", time_taken);

}
