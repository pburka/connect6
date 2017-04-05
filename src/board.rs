pub const BOARD_SIZE : usize = 19;

pub const CENTER : usize = BOARD_SIZE / 2;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Piece {
	Empty,
	Black,
	White,
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
	size : usize,
	cells : [Piece; BOARD_SIZE]
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
	fn get(&self, col:usize) -> Piece {
		self.cells[col]
	}
	
	fn set(&mut self, col:usize, val:Piece) {
		self.cells[col] = val
	}
	
	fn empty() -> Row {
		Row { cells: [Piece::Empty; BOARD_SIZE] }
	}
}

impl Board {
	// coordinates are relative to the lower left corner
	pub fn get(&self, row:usize, col:usize) -> Piece {
		if row >= BOARD_SIZE || col >= BOARD_SIZE {
			panic!("({}, {}) out of range", row, col)
		}
		self.rows[row].get(col)
	}
	
	pub fn set(&mut self, row:usize, col:usize, val:Piece) {
		self.rows[row].set(col, val)
	}
	
	fn get_row(&self, row:usize) -> Line {
		Line { 
			size: BOARD_SIZE, 
			cells : self.rows[row].cells, 
		}
	}
	
	fn get_col(&self, col:usize) -> Line {
		let mut line = Line::empty(BOARD_SIZE);
		for row in 0..BOARD_SIZE {
			let val = self.get(row, col);
			line.set(row, val);
		}
		return line
	}

	/**
	 * Get a diagonal line starting at row, col, and moving
	 * towards the bottom of the board and to the right.
	 * One of row or col must be zero. If both are zero, this
	 * is the main diagonal.
	 */
	fn get_down_diagonal(&self, row:usize, col:usize) -> Line {
		assert!(row == 0 || col == 0);
		assert!(row < BOARD_SIZE);
		assert!(col < BOARD_SIZE);
		let len = BOARD_SIZE - row - col;
		let mut line = Line::empty(len);
		for i in 0..len {
			let val = self.get(row + i, col + i);
			line.set(i, val);
		}
		return line
	}
	
	/**
	 * Get a diagonal line starting at row, col, and moving
	 * towards the top of the board and to the right.
	 * Either col must be zero, or row must be BOARD_SIZE-1. 
	 * If col is zero and row is BOARD_SIZE, this is the 
	 * anti-diagonal.
	 */
	fn get_up_diagonal(&self, row:usize, col:usize) -> Line {
		assert!(row == BOARD_SIZE-1 || col == 0);
		assert!(row < BOARD_SIZE);
		assert!(col < BOARD_SIZE);
		let len = row - col + 1;
		let mut line = Line::empty(len);
		for i in 0..len {
			let val = self.get(row - i, col + i);
			line.set(i, val);
		}
		return line
	}

	pub fn empty() -> Board {
		Board { rows : [Row::empty(); BOARD_SIZE] }
	}
}

impl Line {
	fn empty(size:usize) -> Line {
		Line { size: size, cells: [Piece::Empty; BOARD_SIZE] }
	}
	
	fn set(&mut self, index:usize, val:Piece) {
		assert!(index < self.size);
		self.cells[index] = val
	}
	
	pub fn size(&self) -> usize {
		self.size
	}
}

#[cfg(test)]
mod test {
	use super::Board;
	use super::Piece;
	use board::CENTER;
	use board::BOARD_SIZE;

    #[test]
	fn test_get() {
		let b = Board::empty();
		assert_eq!(b.get(0, 0), Piece::Empty);		
		for r in 0..BOARD_SIZE {
			for c in 0..BOARD_SIZE {
				assert_eq!(b.get(r, c), Piece::Empty);
			}
		}
	}
	
	#[test]
	fn test_set() {
		let mut b = Board::empty();
		b.set(CENTER,   CENTER,   Piece::Black);
		b.set(CENTER+1, CENTER+1, Piece::White);
		b.set(CENTER,   CENTER-1, Piece::White);
		let b = b;
		
		// check the nine center pieces
		// x==1
		assert_eq!(b.get(CENTER+1, CENTER-1), Piece::Empty);
		assert_eq!(b.get(CENTER+1, CENTER  ), Piece::Empty);
		assert_eq!(b.get(CENTER+1, CENTER+1), Piece::White);
		// x==0
		assert_eq!(b.get(CENTER  , CENTER-1), Piece::White);
		assert_eq!(b.get(CENTER  , CENTER  ), Piece::Black);
		assert_eq!(b.get(CENTER  , CENTER+1), Piece::Empty);
		// x==-1
		assert_eq!(b.get(CENTER-1, CENTER-1), Piece::Empty);
		assert_eq!(b.get(CENTER-1, CENTER  ), Piece::Empty);
		assert_eq!(b.get(CENTER-1, CENTER+1), Piece::Empty);
		// check the rest of the board
		for r in 0..BOARD_SIZE {
			for c in 0..BOARD_SIZE {
				if (r > CENTER+1 || r < CENTER-1) && (c > CENTER+1 || c < CENTER-1) {
					assert_eq!(b.get(r, c), Piece::Empty);
				}		
			}
		}
	}
	
	#[test]
	fn test_get_down_diagonal() {
		let b = Board::empty();
		
		assert_eq!(b.get_down_diagonal(0, 0).size(), BOARD_SIZE);
		assert_eq!(b.get_down_diagonal(1, 0).size(), BOARD_SIZE-1);
		assert_eq!(b.get_down_diagonal(0, 1).size(), BOARD_SIZE-1);
		assert_eq!(b.get_down_diagonal(BOARD_SIZE-1, 0).size(), 1);
	}
	
	#[test]
	fn test_get_up_diagonal() {
		let b = Board::empty();
		
		assert_eq!(b.get_up_diagonal(0, 0).size(), 1);
		assert_eq!(b.get_up_diagonal(1, 0).size(), 2);
		assert_eq!(b.get_up_diagonal(BOARD_SIZE-1, 0).size(), BOARD_SIZE);
		assert_eq!(b.get_up_diagonal(BOARD_SIZE-1, BOARD_SIZE-1).size(), 1);
	}
}