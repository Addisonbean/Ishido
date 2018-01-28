extern crate cursive;
extern crate rand;
#[macro_use]
extern crate lazy_static;

mod stone;
mod board;

use cursive::Cursive;

use board::Board;

pub fn start_game(s: &mut Cursive) {
    s.pop_layer();

    let board = Board::new();
    s.add_layer(board);
}

