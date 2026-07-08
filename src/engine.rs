use crate::{
    board::{self, Board, Move},
    piece::{self, Piece},
};

pub struct Engine;

#[derive(Debug, Clone)]
pub struct Line {
    pub moves: Vec<Move>,
    score: f32,
}

impl Line {
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            score: 0.0,
        }
    }
}

impl Engine {
    pub fn slove(board: Board, pieces: Vec<Piece>) -> Line {
        return Self::recurse(board, pieces, Line::new());
    }
    fn recurse(board: Board, pieces: Vec<Piece>, line: Line) -> Line {
        if pieces.len() == 0 {
            return line;
        }
        let mut best_line = line.clone();
        for (i, piece) in pieces.iter().enumerate() {
            let moves = board.get_possible_moves(&piece);
            for m in moves {
                let mut current_line = line.clone();
                let mut current_board = board;

                current_board.apply_move(&m);
                current_line.score += m.score;
                current_line.moves.push(m);

                let mut new_pieces = pieces.clone();
                new_pieces.remove(i);

                let res = Self::recurse(current_board, new_pieces, current_line);
                let score = res.score;

                if score > best_line.score {
                    best_line = res;
                }
            }
        }
        return best_line;
    }
}
