use std::fmt;

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
        Line::on(self, row, 0, 0, 1, BOARD_SIZE)
    }
    
    fn get_col(&self, col:usize) -> Line {
        Line::on(self, 0, col, 1, 0, BOARD_SIZE)
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
        Line::on(self, row, col, 1, 1, len)
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
        Line::on(self, row, col, -1, 1, len)
    }

    pub fn empty() -> Board {
        Board { rows : [Row::empty(); BOARD_SIZE] }
    }
}

impl Line {
    fn empty(size:usize) -> Line {
        assert!(size <= BOARD_SIZE);
        assert!(size >= 1);
        Line { size: size, cells: [Piece::Empty; BOARD_SIZE] }
    }
    
    fn on(board:&Board, row:usize, col:usize, rstride:i32, cstride:i32, size:usize) -> Line {
        let mut line = Line::empty(size);
        for i in 0..size {
            let val = board.get(
                (row as i32 + (i as i32 * rstride)) as usize, 
                (col as i32 + (i as i32 * cstride)) as usize
            );
            line.set(i, val);
        }
        line
    }
    
    // primarily for testing, at least for now
    fn of(s:&str) -> Line {
        let mut line = Line::empty(s.len());
        for (i, c) in s.chars().enumerate() {
            match c {
                '-' => line.set(i, Piece::Empty),
                'O' => line.set(i, Piece::White),
                'X' => line.set(i, Piece::Black),
                _ => panic!("Line strings must consist only of -, O and X (not {})", c)
            }
        }
        line
    }
    
    fn set(&mut self, index:usize, val:Piece) {
        assert!(index < self.size);
        self.cells[index] = val
    }
    
    fn get(&self, index:usize) -> Piece {
        self.cells[index]
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
}

impl fmt::Display for Line {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str("["));
        for i in 0..self.size() {
	        match self.get(i) {
		        Piece::Empty => try!(fmt.write_str("-")),
	            Piece::White => try!(fmt.write_str("O")),
                Piece::Black => try!(fmt.write_str("X")),
            }
        }
        try!(fmt.write_str("]"));
	    Ok(())
    }
}

#[derive(Debug)]
enum State {
    Row(usize),
    Col(usize),
    DownDiag(usize, usize), 
    UpDiag(usize, usize),
    Finished,
}

struct LineIterator<'a> {
    board:&'a Board,
    state:State,
}

impl<'a> LineIterator<'a> {
    fn on(board:&Board) -> LineIterator {
        LineIterator { board: board, state: State::Row(0) }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Line;
    
    fn next(&mut self) -> Option<Line> {
        let result = match self.state {
            State::Row(r) => Some(self.board.get_row(r)),
            State::Col(c) => Some(self.board.get_col(c)),
            State::UpDiag(r, c) => Some(self.board.get_up_diagonal(r, c)),
            State::DownDiag(r, c) => Some(self.board.get_down_diagonal(r, c)),
            State::Finished => None
        };
        self.state = self.state.next();
        return result
    }
}

impl State {
    fn next(&self) -> State {
        const MAX:usize = BOARD_SIZE - 1;
        match self {
            // 0->MAX
            &State::Row(MAX) => State::Col(0), 
            &State::Row(i)   => State::Row(i+1),

            // 0->MAX
            &State::Col(MAX) => State::UpDiag(0, 0),
            &State::Col(i)   => State::Col(i+1),

            // (0,0)->(MAX,0)->(MAX,MAX)
            &State::UpDiag(MAX, 0)   => State::UpDiag(MAX, 1),
            &State::UpDiag(MAX, MAX) => State::DownDiag(0, MAX),
            &State::UpDiag(r, 0)     => State::UpDiag(r+1, 0),
            &State::UpDiag(MAX, c)   => State::UpDiag(MAX, c+1),

            // (0,MAX)->(0,0)->(MAX,0)
            &State::DownDiag(0, 0)   => State::DownDiag(1, 0),
            &State::DownDiag(MAX, 0) => State::Finished,
            &State::DownDiag(0, c)   => State::DownDiag(0, c-1),
            &State::DownDiag(r, 0)   => State::DownDiag(r+1, 0),

            // terminal
            &State::Finished => State::Finished,
            
            _ => panic!("Illegal state {:?}", self)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Board;
    use super::Piece;
    use super::State;
    use super::Line;
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
    
    #[test]
    fn test_state_transition() {
        let mut s = State::Row(0);
        let mut rows = 0;
        let mut cols = 0;
        let mut d1 = 0;
        let mut d2 = 0;
        
        loop {
            match s {
                State::Row(_) => rows += 1,
                State::Col(_) => cols += 1,
                State::UpDiag(_, _) => d1 += 1,
                State::DownDiag(_, _) => d2 += 1,
                State::Finished => break,
            }
            s = s.next();
        }
        
        assert_eq!(rows, BOARD_SIZE);
        assert_eq!(cols, BOARD_SIZE);
        assert_eq!(d1, BOARD_SIZE * 2 - 1);
        assert_eq!(d2, BOARD_SIZE * 2 - 1);        
    }
    
    #[test]
    fn test_line_to_string() {
        assert_eq!(
            Line::of("---").to_string(), 
            "[---]");
        assert_eq!(
            Line::of("---XX-----").to_string(), 
            "[---XX-----]");
        assert_eq!(
            Line::of("---XXO--OOOO--").to_string(), 
            "[---XXO--OOOO--]");
    }
}