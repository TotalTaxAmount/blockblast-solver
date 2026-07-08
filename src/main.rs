use crate::{
    engine::Engine,
    piece::{Piece, pieces},
};

mod board;
mod engine;
mod piece;

fn main() {
    let mut board = board::Board::from(16566183258635535104);
    let pieces = vec![
        Piece::from(1082401), // 5 tall
        Piece::from(1082400), // 4 tall
        Piece::from(1082400), // 4 tall
    ];

    let line = Engine::slove(board, pieces);
    println!("{:?}", line);
    for m in line.moves {
        board.apply_move(&m);
        println!("{}", board);
    }
}
