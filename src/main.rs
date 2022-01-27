use std::io::Write;

const MAX_ROW: i8 = 3;

#[derive(Debug)]
pub enum ClaimType {
    Unclaimed,
    Player1,
    Player2,
}

fn main() {
    let mut curr_turn = 0;

    println!("Welcome to Tic-Tac-Toe!");
    println!("Controls: ");
    println!("When asked, enter either a number from 1-9 (i.e 7 for the bottom left square), or a grid (i.e 3, 3 for the bottom right square)");

    let mut game_board: Vec<ClaimType> = Vec::new();
    for _ in 1..=9 {
        game_board.push(ClaimType::Unclaimed);
    }
    // loop {
    curr_turn += 1;
    draw(&game_board, &curr_turn);
    input(curr_turn % 2 == 0, &mut game_board); // even == player2, odd = player1
                                                // }
}

fn draw(game_board: &Vec<ClaimType>, curr_turn: &i8) {
    //println!("Current turn: {}", curr_turn);
    println!("{:?}", game_board);
    let mut game_string = String::new();
    let mut counter = 0;
    for i in game_board.iter() {
        counter += 1;
        game_string.push('[');
        if matches!(i, ClaimType::Player1) {
            // true = taken up, false = empty
            game_string.push('X');
        } else if matches!(i, &ClaimType::Player2) {
            game_string.push('O');
        } else {
            game_string.push(' ');
        }
        game_string.push(']');
        if counter == MAX_ROW {
            game_string.push('\n');
            counter = 0;
        } else {
            game_string.push(' ');
        }
    }
    println!("{}", game_string);
}

fn input(p2: bool, game_board: &mut Vec<ClaimType>) {
    let input: i8;
    loop {
        let mut temp_string = String::new();
        print!("Enter a location to mark: ");
        std::io::stdout()
            .flush()
            .expect("Could not flush stdout buffer!");
        std::io::stdin()
            .read_line(&mut temp_string)
            .expect("Couldn't read line!");
        input = match temp_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        break;
    }

    game_board[input] = match p2 {
        true => ClaimType::Player2,
        false => ClaimType::Player1,
    };
}
