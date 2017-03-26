pub const BOARD_SIZE : usize = 19;

const CENTER : i32 = 9;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Piece {
	Empty,
	Black,
	White,
}

#[derive(Copy, Clone)]
struct Row {
	cells : [Piece; BOARD_SIZE]
}

#[derive(Copy, Clone)]
pub struct Board {
	rows : [Row; BOARD_SIZE]
}

impl Row {
	fn get(&self, col:i32) -> Piece {
		self.cells[(CENTER + col) as usize]
	}
	
	fn set(&mut self, col:i32, val:Piece) {
		self.cells[(CENTER + col) as usize] = val
	}
	
	fn empty() -> Row {
		Row { cells: [Piece::Empty; BOARD_SIZE] }
	}
}

impl Board {
	// coordinates are relative to the center
	pub fn get(&self, row:i32, col:i32) -> Piece {
		self.rows[(CENTER + row) as usize].get(col)
	}
	
	pub fn set(&mut self, row:i32, col:i32, val:Piece) {
		self.rows[(CENTER + row) as usize].set(col, val)
	}
	
	pub fn empty() -> Board {
		Board { rows : [Row::empty(); BOARD_SIZE] }
	}
}

#[cfg(test)]
mod test {
	use super::Board;
	use super::Piece;

    #[test]
	fn test_get() {
		let b = Board::empty();
		assert_eq!(b.get(0, 0), Piece::Empty);		
		for r in -9..10 {
			for c in -9..10 {
				assert_eq!(b.get(r, c), Piece::Empty);
			}
		}
	}
	
	#[test]
	fn test_set() {
		let mut b = Board::empty();
		b.set(0, 0, Piece::Black);
		b.set(1, 1, Piece::White);
		b.set(0, -1, Piece::White);
		let b = b;
		
		// check the nine center pieces
		// x==1
		assert_eq!(b.get( 1,-1), Piece::Empty);
		assert_eq!(b.get( 1, 0), Piece::Empty);
		assert_eq!(b.get( 1, 1), Piece::White);
		// x==0
		assert_eq!(b.get( 0,-1), Piece::White);
		assert_eq!(b.get( 0, 0), Piece::Black);
		assert_eq!(b.get( 0, 1), Piece::Empty);
		// x==-1
		assert_eq!(b.get(-1,-1), Piece::Empty);
		assert_eq!(b.get(-1, 0), Piece::Empty);
		assert_eq!(b.get(-1, 1), Piece::Empty);
		// check the rest of the board
		for r in -9..10 {
			for c in -9..10 {
				if (r > 1 || r < -1) && (c > 1 || c < -1) {
					assert_eq!(b.get(r, c), Piece::Empty);
				}		
			}
		}
	}
}