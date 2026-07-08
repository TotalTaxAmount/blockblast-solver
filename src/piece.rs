use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub u32);
impl Piece {
    pub fn get_index(row: u8, col: u8) -> u8 {
        (row * 5 + col) as u8
    }

    pub fn is_occupied(&self, row: u8, col: u8) -> bool {
        let index = Self::get_index(row, col);
        ((self.0 >> index) & 1) == 1
    }

    pub fn get_anchor(&self) -> Option<(i8, i8)> {
        for row in 0..5 {
            for col in 0..5 {
                if self.is_occupied(row, col) {
                    return Some((row as i8, col as i8));
                }
            }
        }
        None
    }
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
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

impl From<u32> for Piece {
    fn from(value: u32) -> Self {
        Piece(value)
    }
}

pub mod pieces {
    // TODO: Finish this if needed
    use super::Piece;
    pub const FIVE_LONG: Piece = Piece(0b11111_00000_00000_00000_00000);
    pub const FIVE_TALL: Piece = Piece(0b10000_10000_10000_10000_10000);
    pub const THREE_LONG: Piece = Piece(0b11100_00000_00000_00000_00000);
    pub const THREE_TALL: Piece = Piece(0b10000_10000_10000_00000_00000);
    pub const THREE_BY_THREE: Piece = Piece(0b11100_11100_11100_00000_00000);
    pub const THREE_BY_TWO_LONG: Piece = Piece(0b11100_11100_00000_00000_00000);
    pub const THREE_BY_TWO_TALL: Piece = Piece(0b11000_11000_11000_00000_00000);
    pub const BIG_L_NE: Piece = Piece(0b10000_10000_11100_00000_00000);
    pub const BIG_L_NW: Piece = Piece(0b11100_10000_10000_00000_00000);
    pub const BIG_L_SW: Piece = Piece(0b10000_10000_11100_00000_00000);
}
