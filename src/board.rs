use std::fmt::{self, Write};

use crate::{board, piece};

const COLUMN_MASKS: [u64; 8] = [
    0x0101010101010101,      // Column 0
    0x0101010101010101 << 1, // Column 1
    0x0101010101010101 << 2, // Column 2
    0x0101010101010101 << 3, // Column 3
    0x0101010101010101 << 4, // Column 4
    0x0101010101010101 << 5, // Column 5
    0x0101010101010101 << 6, // Column 6
    0x0101010101010101 << 7, // Column 7
];

pub const ROW_MASKS: [u64; 8] = [
    0x00000000000000FF, // Row 0
    0x000000000000FF00, // Row 1
    0x0000000000FF0000, // Row 2
    0x00000000FF000000, // Row 3
    0x000000FF00000000, // Row 4
    0x0000FF0000000000, // Row 5
    0x00FF000000000000, // Row 6
    0xFF00000000000000, // Row 7
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClearType {
    Row(u8),
    Column(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Board(pub u64);
impl Board {
    pub fn new() -> Self {
        Self(0)
    }
    #[inline]
    fn get_index(row: u8, col: u8) -> u8 {
        debug_assert!(row < 8 && col < 8, "Coordinates must be between 0 and 7");
        (row * 8) + col
    }

    #[inline]
    fn get_pos_mask(row: u8, col: u8) -> u64 {
        1 << Self::get_index(row, col)
    }

    pub fn set_square(&mut self, row: u8, col: u8) {
        let index = Self::get_index(row, col);
        self.0 |= 1 << index;
    }

    pub fn clear_square(&mut self, row: u8, col: u8) {
        let index = Self::get_index(row, col);
        self.0 &= !(1 << index);
    }

    pub fn is_occupied(&self, row: u8, col: u8) -> bool {
        let index = Self::get_index(row, col);
        ((self.0 >> index) & 1) == 1
    }

    pub fn apply_move(&mut self, m: &Move) {
        let anchor = m.piece.get_anchor();
        let Some((anchor_row, anchor_col)) = anchor else {
            return;
        };

        let (target_row, target_col) = m.pos;

        for r in 0..5 {
            for c in 0..5 {
                if m.piece.is_occupied(r, c) {
                    let board_row = target_row as i8 + r as i8 - anchor_row;
                    let board_col = target_col as i8 + c as i8 - anchor_col;
                    if board_row >= 0 && board_row < 8 && board_col >= 0 && board_col < 8 {
                        self.set_square(board_row as u8, board_col as u8);
                    }
                }
            }
        }

        for clear_type in &m.clear {
            match clear_type {
                ClearType::Row(row) => {
                    let mask = !ROW_MASKS[*row as usize];
                    self.0 &= mask;
                }
                ClearType::Column(col) => {
                    let mask = !COLUMN_MASKS[*col as usize];
                    self.0 &= mask;
                }
            }
        }
    }

    // Returns a list of possible moves for the given piece on this board.
    pub fn get_possible_moves(&self, piece: &piece::Piece) -> Vec<Move> {
        let mut moves = Vec::new();
        let Some((anchor_row, anchor_col)) = piece.get_anchor() else {
            return moves;
        };

        for row in 0..8 {
            for col in 0..8 {
                let row_offset = row as i8 - anchor_row;
                let col_offset = col as i8 - anchor_col;
                let mut is_valid = true;
                let mut clear = Vec::new();
                let mut placement_mask = 0;

                for r in 0..5 {
                    for c in 0..5 {
                        if piece.is_occupied(r, c) {
                            let board_row = (row_offset + r as i8) as u8;
                            let board_col = (col_offset + c as i8) as u8;
                            if board_row >= 8 || board_col >= 8 {
                                is_valid = false;
                                break;
                            }

                            if self.is_occupied(board_row, board_col) {
                                is_valid = false;
                                break;
                            }

                            placement_mask |= Self::get_pos_mask(board_row, board_col);
                            if (self.0 & COLUMN_MASKS[board_col as usize])
                                | (placement_mask & COLUMN_MASKS[board_col as usize])
                                == COLUMN_MASKS[board_col as usize]
                            {
                                clear.push(ClearType::Column(board_col));
                            }

                            if (self.0 & ROW_MASKS[board_row as usize])
                                | (placement_mask & ROW_MASKS[board_row as usize])
                                == ROW_MASKS[board_row as usize]
                            {
                                clear.push(ClearType::Row(board_row));
                            }
                        }
                    }
                }

                if is_valid {
                    let num_clears = clear.len();
                    moves.push(Move {
                        piece: *piece,
                        pos: (row as u8, col as u8),
                        clear,
                        score: if num_clears == 0 {
                            piece.0.count_ones() as f32
                        } else {
                            8.0 * num_clears as f32
                        },
                    });
                }
            }
        }

        moves
    }
}

impl From<u64> for Board {
    fn from(value: u64) -> Self {
        Board(value)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                if self.is_occupied(row, col) {
                    let _ = f.write_str("1 ");
                } else {
                    let _ = f.write_str(". ");
                }
            }
            let _ = f.write_char('\n');
        }
        let _ = f.write_char('\n');
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub piece: piece::Piece,
    pub pos: (u8, u8), // of the top-leftmost square of the piece
    pub clear: Vec<ClearType>,
    pub score: f32,
}
