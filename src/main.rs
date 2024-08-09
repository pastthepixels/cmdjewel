mod game;

use std::io::stdin;

fn main() {
    let mut board = game::Board::new();
    // Simple function to loop through the board.
    loop {
        // 1. Update board
        board.fill_from_top();
        board.update_physics_frame();
        // 2. Print board
        print_board(&board);
        // 3. Handle commands
        let mut command = String::new();
        stdin()
            .read_line(&mut command)
            .expect("Something went wrong...");
        if &command == "test\n" {
            println!("Hello world!");
        }
    }
}

fn print_board(board: &game::Board) {
    let mut count = 0;
    board.as_ref().iter().for_each(|x| {
        count += 1;
        let string = match x {
            game::Gems::Empty => ".",
            game::Gems::Blue => "B",
            game::Gems::White => "W",
            game::Gems::Red => "R",
            game::Gems::Yellow => "Y",
            game::Gems::Green => "G",
            game::Gems::Orange => "O",
            game::Gems::Purple => "P",
        };
        if count == 8 {
            println!(" {} ", string);
            count = 0;
        } else {
            print!(" {} ", string);
        }
    });
}
