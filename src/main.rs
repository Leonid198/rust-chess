use std::io;

#[derive(Debug, PartialEq)]
enum BoardError {
    WrongLength,
    InvalidReference,
    EmptyCell,
    InvalidMove,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn display_debug(&self) {
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

    fn get_cell_index(&self, cell: &str) -> Result<(usize, usize), BoardError> {
        if cell.chars().count() != 2 {
            return Err(BoardError::WrongLength);
        }
        Ok((
            self.get_column_index(cell.chars().nth(0))?,
            self.get_row_index(cell.chars().nth(1))?,
        ))
    }

    fn get_column_index(&self, column: Option<char>) -> Result<usize, BoardError> {
        let column = match column {
            Some(x) => x as usize - 65, // 65 is A in unicode, this translates A to 0, B to 1, etc.
            None => return Err(BoardError::WrongLength),
        };
        if column >= self.board.len() {
            return Err(BoardError::InvalidReference);
        };
        Ok(column)
    }

    fn get_row_index(&self, row: Option<char>) -> Result<usize, BoardError> {
        let row = match row {
            Some(x) => x as usize - 48 - 1, // 48 is 0 in unicode, this converts the number characters to an integer.
            None => return Err(BoardError::WrongLength),
        };
        if row >= self.board[0].len() {
            return Err(BoardError::InvalidReference);
        }
        Ok(row)
    }

    fn get_cell(&self, cell: &str) -> Result<&Option<(Colour, Piece)>, BoardError> {
        if cell.chars().count() != 2 {
            return Err(BoardError::WrongLength);
        }
        let column = self.get_column(cell.chars().nth(0))?;
        let row = self.get_row_index(cell.chars().nth(1))?;
        Ok(&column[row])
    }

    fn get_column(
        &self,
        column: Option<char>,
    ) -> Result<&[Option<(Colour, Piece)>; 8], BoardError> {
        let column = self.get_column_index(column)?;
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

    fn set_cell(&mut self, cell: &str, data: Option<(Colour, Piece)>) -> Result<(), BoardError> {
        let cell = self.get_cell_index(cell)?;
        self.board[cell.0][cell.1] = data;
        Ok(())
    }

    fn board_move(&mut self, from_cell: &str, to_cell: &str) -> Result<(), BoardError> {
        self.set_cell(to_cell, *self.get_cell(from_cell)?)?;
        self.set_cell(from_cell, None)?;
        Ok(())
    }

    fn validated_move(&mut self, from_cell: &str, to_cell: &str) -> Result<(), BoardError> {
        let from_cell_index = self.get_cell_index(from_cell)?;
        let to_cell_index = self.get_cell_index(to_cell)?;
        let (colour, piece) = match self.get_cell(from_cell)? {
            Some(x) => x,
            None => return Err(BoardError::EmptyCell),
        };

        match piece {
            Piece::Pawn => match colour {
                Colour::White => {
                    let range = if from_cell_index.1 == 1 { 2 } else { 1 };
                    let distance = to_cell_index.1 as i8 - from_cell_index.1 as i8;
                    let lateral = to_cell_index.0 as i8 - from_cell_index.0 as i8;
                    if lateral.abs() <= 1 && 0 < distance && distance <= range {
                        if lateral == 0 && !check_collision(from_cell_index + 1, to_cell_index)? {}
                    } else {
                        return Err(BoardError::InvalidMove);
                    }
                }
                Colour::Black => unimplemented!(),
            },
            Piece::Knight => unimplemented!(),
            Piece::Bishop => unimplemented!(),
            Piece::Rook => unimplemented!(),
            Piece::Queen => unimplemented!(),
            Piece::King => unimplemented!(),
        }
        Ok(())
    }

    fn check_collision(
        &self,
        from_cell_index: usize,
        to_cell_index: usize,
    ) -> Result<bool, BoardError> {
        unimplemented!()
    }
}

fn main() {
    let mut board = Board::new();
    board.display_debug();
    loop {
        let mut player_move = String::new();
        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");
        match board.board_move(&player_move[..2], &player_move[3..5]) {
            Ok(()) => (),
            Err(x) => println!("{x:?}"),
        }
        board.display_debug();
    }
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
        assert_eq!(board.get_cell("N1"), Err(BoardError::InvalidReference));
        assert_eq!(board.get_cell("I9"), Err(BoardError::InvalidReference));
        assert_eq!(board.get_cell("C3").unwrap(), &None);
    }
    #[test]
    fn set_cell() {
        let mut board = Board::new();
        board.set_cell("A1", None).unwrap();
        assert_eq!(board.get_cell("A1").unwrap(), &None);
    }
    #[test]
    fn board_move() {
        let mut board = Board::new();
        board.board_move("E1", "E3").unwrap();
        assert_eq!(board.get_cell("E1").unwrap(), &None);
        assert_eq!(
            board.get_cell("E3").unwrap(),
            &Some((Colour::White, Piece::King))
        );
    }
}
