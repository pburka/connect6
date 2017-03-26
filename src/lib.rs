#![allow(dead_code)]

mod board;

#[cfg(test)]
mod test {
	use super::board::Board;
	use super::board::Piece;

    #[test]
	fn test_get() {
		let b = Board::empty();
		assert_eq!(b.get(0, 0), Piece::Empty);
	}
}