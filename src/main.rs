mod game;
use std::io;
use game::new_game;
use std::collections::HashMap;


fn main() {
    //print_board((game::BOTTOMLEFT_BORDER, 0))
    //states_search(10);
    ab_search(18);
    //play_game()
    //println!("{}", game::count_bits(9))
}

fn play_game(){
    let mut game = new_game();
    let mut input = String::new();
    let mut position: u64;
    for move_nbr in 0..32{
        print_board(game);
        if move_nbr % 2 == 0 {
            let pseudo_legal = game::create_potential_moves_mask(game);
            let legal_moves = game::process_pseudo_legal_moves(game, pseudo_legal);
            let mut legal_mask: u64 = 0;
            for mv in legal_moves{
                legal_mask = legal_mask | mv.played_piece;
            }
            print_board((legal_mask, 0));
        } else {
            let pseudo_legal = game::create_potential_moves_mask(game::reverse(game));
            let legal_moves = game::process_pseudo_legal_moves(game::reverse(game), pseudo_legal);
            let mut legal_mask: u64 = 0;
            for mv in legal_moves{
                legal_mask = legal_mask | mv.played_piece;
            }
            print_board((legal_mask, 0));
        }
        println!("Enter your next move (5a for instance): ");
        io::stdin().read_line(&mut input).expect("Failed to read");
        position = convert(&input);
        input = "".to_string();
        if move_nbr % 2 == 0 {
            game = game::play_move(game, position);
        } else {
            game = game::reverse(game::play_move(game::reverse(game), position));
        }
    }
    print_board(game);
}

fn states_search(depth: u8){
    let mut POSITIONS: HashMap<u128, bool> = HashMap::new();
    unsafe {
        game::TOTAL_MOVES = 0;
        game::EXPLORED_MOVES = 0;
    }
    let start = game::Move {
        game : new_game(),
        played_piece : 0,
        next : Vec::new(),
    };
    use std::time::Instant;
    let now = Instant::now();
    game::depth_search(start, depth);
    let elapsed = now.elapsed();
    unsafe{
        println!("--------------------------------------------------------------------\n\tDepth : {}
                 \n\tExplored : {}\n\tTotal : {}\n\tElapsed : {:.3?}\n\tMoves per second : {}\n\tAverage Moves per position : {}\n", 
        depth, game::EXPLORED_MOVES, game::TOTAL_MOVES, elapsed, game::TOTAL_MOVES/(elapsed.as_millis() as u64)*1000, game::TOTAL_MOVES/game::EXPLORED_MOVES);
    }
}

fn ab_search(depth: u8){
    let mut POSITIONS: HashMap<u128, bool> = HashMap::new();
    unsafe {
        game::TOTAL_MOVES = 0;
        game::EXPLORED_MOVES = 0;
    }
    let start = game::Move {
        game : new_game(),
        played_piece : 0,
        next : Vec::new(),
    };
    use std::time::Instant;
    let now = Instant::now();
    game::alphabeta(start, -1000, depth, &mut POSITIONS);
    let elapsed = now.elapsed();
    unsafe{
        println!("--------------------------------------------------------------------\n\tDepth : {}
                 \n\tExplored : {}\n\tTotal : {}\n\tElapsed : {:.3?}\n\tMoves per second : {}\n\tAverage Moves per position : {}\n", 
        depth, game::EXPLORED_MOVES, game::TOTAL_MOVES, elapsed, game::TOTAL_MOVES/(elapsed.as_millis() as u64)*1000, game::TOTAL_MOVES/game::EXPLORED_MOVES);
    }
}

fn convert(position: &str) -> u64{
    let column: u8 = position.chars().nth(1).unwrap() as u8 - 97;
    let line: u8 = position.chars().nth(0).unwrap() as u8 - 49;
    let offset = line * 6 + column;
    let pos: u64 = 1<<offset;
    pos
}

fn print_board(board:(u64, u64)){
    for x in 0..6{
        print!("{}", x+1);
        for y in 0..6{
            if (board.0 >> (x*6+y))% 2 == 1 {
                print!(" X")
            } else if (board.1 >> (x*6+y))% 2 == 1 {
                print!(" O")
            } else {
                print!{" ."}
            }
        }
        println!("");
    }
    println!("  a b c d e f");
    println!("Black : {} White : {}", game::count_bits(board.0), game::count_bits(board.0))
}
