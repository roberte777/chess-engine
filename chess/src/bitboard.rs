use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct BitBoard(pub u64);

pub const EMPTY: BitBoard = BitBoard(0);
// Impl BitAnd
impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitAnd<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitOr<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitXor<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitAndAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: &BitBoard) {
        self.0 &= other.0;
    }
}

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitOrAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: &BitBoard) {
        self.0 |= other.0;
    }
}

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl BitXorAssign<&BitBoard> for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: &BitBoard) {
        self.0 ^= other.0;
    }
}

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<&BitBoard> for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Mul<BitBoard> for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl fmt::Display for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if x % 8 == 7 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

impl BitBoard {
    /// Construct a new bitboard from a u64
    #[inline]
    pub fn new(b: u64) -> BitBoard {
        BitBoard(b)
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn set(rank: u8, file: u8) -> BitBoard {
        let sq = rank * 8 + file;
        BitBoard::from_square(sq)
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    #[inline]
    pub fn from_square(index: u8) -> BitBoard {
        BitBoard(1u64 << index)
    }

    /// Convert an `Option<Square>` to an `Option<BitBoard>`
    #[inline]
    pub fn from_maybe_square(sq: Option<u8>) -> Option<BitBoard> {
        sq.map(BitBoard::from_square)
    }

    /// Convert a `BitBoard` to a `Square`.  This grabs the least-significant `Square`
    #[inline]
    pub fn to_square(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    /// Count the number of `Squares` set in this `BitBoard`
    #[inline]
    pub fn popcnt(&self) -> u32 {
        self.0.count_ones()
    }

    /// Reverse this `BitBoard`.  Look at it from the opponents perspective.
    #[inline]
    pub fn reverse_colors(&self) -> BitBoard {
        BitBoard(self.0.swap_bytes())
    }

    /// Convert this `BitBoard` to a `usize` (for table lookups)
    #[inline]
    pub fn to_size(self, rightshift: u8) -> usize {
        (self.0 >> rightshift) as usize
    }
}

/// For the `BitBoard`, iterate over every `Square` set.
impl Iterator for BitBoard {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= BitBoard::from_square(result);
            Some(result)
        }
    }
}
