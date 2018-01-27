extern crate cursive;

mod stone;
mod board;

use cursive::Cursive;

use board::Board;

pub fn start_game(s: &mut Cursive) {
    s.pop_layer();

    let mut board = Board::new();
    board.test();

    s.add_layer(board);
}

