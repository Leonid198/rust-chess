use std::io;

#[derive(Debug, PartialEq)]
enum BoardError {
    WrongLength,
}

#[derive(Debug, PartialEq)]
enum Colour {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum BoardSquare {
    White,
    Black,
}

trait AsChar {
    fn as_char(&self) -> char;
}

// TODO: Use macro
impl AsChar for (Colour, Piece) {
    fn as_char(&self) -> char {
        match self.0 {
            Colour::White => match self.1 {
                Piece::Pawn => '♙',
                Piece::Knight => '♘',
                Piece::Bishop => '♗',
                Piece::Rook => '♖',
                Piece::Queen => '♕',
                Piece::King => '♔',
            },
            Colour::Black => match self.1 {
                Piece::Pawn => '♟',
                Piece::Knight => '♞',
                Piece::Bishop => '♝',
                Piece::Rook => '♜',
                Piece::Queen => '♛',
                Piece::King => '♚',
            },
        }
    }
}

// TODO: Use macro
impl AsChar for &(Colour, Piece) {
    fn as_char(&self) -> char {
        match self.0 {
            Colour::White => match self.1 {
                Piece::Pawn => '♙',
                Piece::Knight => '♘',
                Piece::Bishop => '♗',
                Piece::Rook => '♖',
                Piece::Queen => '♕',
                Piece::King => '♔',
            },
            Colour::Black => match self.1 {
                Piece::Pawn => '♟',
                Piece::Knight => '♞',
                Piece::Bishop => '♝',
                Piece::Rook => '♜',
                Piece::Queen => '♛',
                Piece::King => '♚',
            },
        }
    }
}

impl AsChar for BoardSquare {
    fn as_char(&self) -> char {
        match self {
            BoardSquare::White => ' ',
            BoardSquare::Black => 'X',
        }
    }
}

struct Board {
    board: [[Option<(Colour, Piece)>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            // [column][row]
            board: [
                [
                    Some((Colour::White, Piece::Rook)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Rook)),
                ],
                [
                    Some((Colour::White, Piece::Knight)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Knight)),
                ],
                [
                    Some((Colour::White, Piece::Bishop)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Bishop)),
                ],
                [
                    Some((Colour::White, Piece::Queen)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Queen)),
                ],
                [
                    Some((Colour::White, Piece::King)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::King)),
                ],
                [
                    Some((Colour::White, Piece::Bishop)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Bishop)),
                ],
                [
                    Some((Colour::White, Piece::Knight)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Knight)),
                ],
                [
                    Some((Colour::White, Piece::Rook)),
                    Some((Colour::White, Piece::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    Some((Colour::Black, Piece::Pawn)),
                    Some((Colour::Black, Piece::Rook)),
                ],
            ],
        }
    }

    pub fn display_debug(&self) {
        println!("┏━┳━┳━┳━┳━┳━┳━┳━┓",);
        for (row, _) in self.board.iter().enumerate() {
            let row_data = self.get_row(row);
            for (column, data) in row_data.iter().enumerate() {
                let value: Box<dyn AsChar> = match data {
                    Some(x) => Box::new(x),
                    None => {
                        if (row + column) % 2 == 0 {
                            Box::new(BoardSquare::Black)
                        } else {
                            Box::new(BoardSquare::White)
                        }
                    }
                };
                print!("┃{}", value.as_char());
            }
            println!("┃");
            if row != self.board[0].len() - 1 {
                println!("┣━╋━╋━╋━╋━╋━╋━╋━┫",);
            }
        }
        println!("┗━┻━┻━┻━┻━┻━┻━┻━┛",);
    }

    fn get_cell(&self, cell: &str) -> Result<&Option<(Colour, Piece)>, BoardError> {
        if cell.chars().count() != 2 {
            return Err(BoardError::WrongLength);
        }
        let column = self.get_column(cell.chars().nth(0))?;
        let row = match cell.chars().nth(1) {
            Some(x) => x as usize - 48, // 48 is 0 in unicode, this converts the number characters to an integer.
            None => return Err(BoardError::WrongLength),
        };

        Ok(&column[row - 1])
    }

    // TODO: Fix
    fn get_column(
        &self,
        column: Option<char>,
    ) -> Result<&[Option<(Colour, Piece)>; 8], BoardError> {
        let column = match column {
            Some(x) => x as usize - 65, // 65 is A in unicode, this translates A to 0, B to 1, etc.
            None => return Err(BoardError::WrongLength),
        };
        Ok(&self.board[column])
    }

    fn get_row(&self, row: usize) -> [&Option<(Colour, Piece)>; 8] {
        let mut output: [&Option<(Colour, Piece)>; 8] =
            [&None, &None, &None, &None, &None, &None, &None, &None];
        for (column, data) in output.iter_mut().enumerate() {
            *data = &self.board[column][row]
        }

        output
    }
}

fn main() {
    let board = Board::new();
    board.display_debug();
}

#[cfg(test)]
mod tests {
    use crate::Board;
    use crate::BoardError;
    use crate::Colour;
    use crate::Piece;
    #[test]
    fn rows() {
        let board = Board::new();
        assert_eq!(
            board.get_row(0),
            [
                &Some((Colour::White, Piece::Rook)),
                &Some((Colour::White, Piece::Knight)),
                &Some((Colour::White, Piece::Bishop)),
                &Some((Colour::White, Piece::Queen)),
                &Some((Colour::White, Piece::King)),
                &Some((Colour::White, Piece::Bishop)),
                &Some((Colour::White, Piece::Knight)),
                &Some((Colour::White, Piece::Rook)),
            ]
        );
        assert_eq!(
            board.get_row(1),
            [
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
                &Some((Colour::White, Piece::Pawn)),
            ]
        );
        for row in 2..6 {
            assert_eq!(
                board.get_row(row),
                [&None, &None, &None, &None, &None, &None, &None, &None,]
            );
        }
        assert_eq!(
            board.get_row(6),
            [
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
                &Some((Colour::Black, Piece::Pawn)),
            ]
        );
        assert_eq!(
            board.get_row(7),
            [
                &Some((Colour::Black, Piece::Rook)),
                &Some((Colour::Black, Piece::Knight)),
                &Some((Colour::Black, Piece::Bishop)),
                &Some((Colour::Black, Piece::Queen)),
                &Some((Colour::Black, Piece::King)),
                &Some((Colour::Black, Piece::Bishop)),
                &Some((Colour::Black, Piece::Knight)),
                &Some((Colour::Black, Piece::Rook)),
            ]
        );
    }

    #[test]
    fn columns() {
        let board = Board::new();
        assert_eq!(
            [
                board.get_column(Some('A')).unwrap(),
                board.get_column(Some('B')).unwrap(),
                board.get_column(Some('C')).unwrap(),
                board.get_column(Some('D')).unwrap(),
                board.get_column(Some('E')).unwrap(),
                board.get_column(Some('F')).unwrap(),
                board.get_column(Some('G')).unwrap(),
                board.get_column(Some('H')).unwrap(),
            ],
            [
                &board.board[0],
                &board.board[1],
                &board.board[2],
                &board.board[3],
                &board.board[4],
                &board.board[5],
                &board.board[6],
                &board.board[7]
            ]
        )
    }
    #[test]
    fn cells() {
        let board = Board::new();
        assert_eq!(
            board.get_cell("A1").unwrap(),
            &Some((Colour::White, Piece::Rook))
        );
        assert_eq!(board.get_cell("N1"), Err(BoardError::WrongLength));
        assert_eq!(board.get_cell("C3").unwrap(), &None);
    }
}
