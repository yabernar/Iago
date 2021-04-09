use std::cmp;

pub fn new_game() -> (u64, u64){
    (68853694464, 34628173824)
}
/*
fn put_piece((player : u64, adversary : u64), position:u64) -> (u64, u64){
    if position & (player | adversary) > 0 { // Position is not already set
        return (player | position, adversary) 
    }
    (0, 0) // Fail state
}*/

// Bitboards with borders set to 1;
pub static TOP_BORDER: u64 = 255;
pub static LEFT_BORDER: u64 = 72340172838076673;
pub static RIGHT_BORDER: u64 = 9259542123273814144;
pub static BOTTOM_BORDER: u64 = 18374686479671623680;
pub static TOPLEFT_BORDER: u64 = TOP_BORDER | LEFT_BORDER;
pub static TOPRIGHT_BORDER: u64 = TOP_BORDER | RIGHT_BORDER;
pub static BOTTOMLEFT_BORDER: u64 = BOTTOM_BORDER | LEFT_BORDER;
pub static BOTTOMRIGHT_BORDER: u64 = BOTTOM_BORDER | RIGHT_BORDER;

pub static mut TOTAL_MOVES: u64 = 0;
pub static mut EXPLORED_MOVES: u64 = 0;

// Assumes the position sent is a legal move
pub fn play_move(game :(u64, u64), position:u64) -> (u64, u64){
    let mut player_next = game.0 | position;

    right_shift_search(TOP_BORDER, game, position, &mut player_next, 8);
    right_shift_search(LEFT_BORDER, game, position, &mut player_next, 1);
    left_shift_search(RIGHT_BORDER, game, position, &mut player_next, 1);
    left_shift_search(BOTTOM_BORDER, game, position, &mut player_next, 8);
    right_shift_search(TOPLEFT_BORDER, game, position, &mut player_next, 9);
    right_shift_search(TOPRIGHT_BORDER, game, position, &mut player_next, 7);
    left_shift_search(BOTTOMLEFT_BORDER, game, position, &mut player_next, 7);
    left_shift_search(BOTTOMRIGHT_BORDER, game, position, &mut player_next, 9);

    // Checking if some pieces have been flipped
    if player_next ^ (game.0 | position) == 0 {
        return (0,0) // Fail state
    }

    // Changing adversary board
    let adversary_next = game.1 & (game.1 ^ player_next); // Removing the flipped pieces with a XOR. As they only appear in both bitboards if they are flipped.
    (player_next, adversary_next)
}

fn right_shift_search(border : u64, game : (u64, u64), position : u64, player_next : &mut u64, shift_value : u8) {
    if position & border == 0{ // Played piece is not on the edge
        let mut current = position >> shift_value;
        let mut flips = 0;
        loop{
            if current & game.0 > 0{ // Piece in searched position is of own color
                *player_next = *player_next | flips;
                break;
            } else if (current & border == 0) && (current & game.1 > 0){ // Piece in searched position is opponent and not on the edge
                flips = flips | current;
                current = current >> shift_value;
                continue;
            } else { // Either no piece in the searched position, or opponent piece is on the edge
                break;
            }
        }
    }
}

fn left_shift_search(border : u64, game : (u64, u64), position : u64, player_next : &mut u64, shift_value : u8) {
    if position & border == 0{ // Played piece is not on the edge
        let mut current = position << shift_value;
        let mut flips = 0;
        loop{
            if current & game.0 > 0{ // Piece in searched position is of own color
                *player_next = *player_next | flips;
                break;
            } else if (current & border == 0) && (current & game.1 > 0){ // Piece in searched position is opponent and not on the edge
                flips = flips | current;
                current = current << shift_value;
                continue;
            } else { // Either no piece in the searched position, or opponent piece is on the edge
                break;
            }
        }
    }
}

pub fn create_potential_moves_mask(game : (u64, u64)) -> u64 {
    let all_pieces = game.0 | game.1;
    all_pieces ^ ((((game.1 << 1) | (game.1 << 9) | (game.1 >> 7) | LEFT_BORDER) ^ LEFT_BORDER) | 
                  (((game.1 >> 1) | (game.1 >> 9) | (game.1 << 7) | RIGHT_BORDER) ^ RIGHT_BORDER) |
                  (game.1 << 8) | (game.1 >> 8) | all_pieces)
}

pub struct Move {
    pub game : (u64, u64),
    pub played_piece: u64,
    pub next : Vec<Move>,
}

pub fn process_pseudo_legal_moves(game: (u64, u64), moves_mask: u64) -> Vec<Move>{
    let mut result : Vec<Move> = Vec::new();
    let mut start = 1;
    for _i in 0..64 {
        if start & moves_mask > 0 {
            let next_game = reverse(play_move(game, start));
            if (next_game.0 | next_game.1) > 0{
                let leaf = Move {
                    game : next_game,
                    played_piece : start,
                    next : Vec::new(),
                };
                result.push(leaf);
            }
        }
        start = start << 1;
    }
    return result
}

pub fn depth_search(mut starting_move : Move, depth: u8){
    let pseudo_legal_moves = create_potential_moves_mask(starting_move.game);
    let legal_moves = process_pseudo_legal_moves(starting_move.game, pseudo_legal_moves);
    unsafe {
        TOTAL_MOVES += legal_moves.len() as u64;
        EXPLORED_MOVES += 1;
    }
    starting_move.next = legal_moves;
    if depth > 0{
        for mv in starting_move.next {
            depth_search(mv, depth-1)
        }
    }
}


pub fn alphabeta(mut starting_move : Move, i : i64, depth : u8) -> i64{
    let pseudo_legal_moves = create_potential_moves_mask(starting_move.game);
    let legal_moves = process_pseudo_legal_moves(starting_move.game, pseudo_legal_moves);
    unsafe {
        TOTAL_MOVES += legal_moves.len() as u64;
        EXPLORED_MOVES += 1;
    }
    starting_move.next = legal_moves;

    if (starting_move.next.len() == 0) || (depth == 0){
        return 100 - (count_bits(starting_move.game.0) as i64 - count_bits(starting_move.game.1) as i64)
    }
    let mut j : i64 = -1000; // maximum is -100 probably
    for mv in starting_move.next {
        j = cmp::max(j, alphabeta(mv, j, depth-1));
        if -j <= i{
           return -j
        }
    }
    return -j
}

pub fn count_bits(v: u64) -> u64{
    let mut c : u64 = 0; // c accumulates the total bits set in v
    let mut tmp : u64 = v;
    while tmp > 0 {
        tmp &= tmp - 1; // clear the least significant bit set
        c += 1
    }
    return c
}

pub fn count_bits_fast_but_not_working(v : u64) -> u64{
    let mut tmp = v - ((v >> 1) & 0x5555555555555555);
    tmp = tmp & 0x3333333333333333 + ((tmp >> 2) & 0x3333333333333333);
    (((tmp + (tmp >> 4)) & 0xF0F0F0F0F0F0F0F) * 0x101010101010101) >> 56 // count
}

pub fn reverse(tuple: (u64, u64)) -> (u64, u64){
    (tuple.1, tuple.0)
}