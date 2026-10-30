use std::fmt;
use std::fmt::Formatter;

pub struct Grid {
    pub squares: [[Square; 3]; 3],
}

pub trait GridHandler: fmt::Display {
    fn is_complete(&self) -> GridResult;
    fn make_move(&mut self, x_coordinate : usize, y_coordinate: usize, state: Mark) -> Result<(), String>;
    fn available_spaces(&self) -> Vec<(usize, usize)>;
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in self.squares.iter() {
            for square in row.iter() {
                let char = match square.state {
                    State::Empty => ".",
                    State::Cross => "X",
                    State::Naught => "O",
                };
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn check_rows_for_winner(&self) -> Option<GridResult> {
        for row in self.squares.iter() {
            if row.iter().all(|square| square.state == State::Cross) {
                return Some(GridResult::CrossWon)
            }
            if row.iter().all(|square| square.state == State::Naught) {
                return Some(GridResult::NaughtWon)
            }
        }

        None
    }

    fn check_columns_for_winner(&self) -> Option<GridResult> {
        for coordinate in 0..3 {
            let column_states = [
                self.squares[0][coordinate].state,
                self.squares[1][coordinate].state,
                self.squares[2][coordinate].state,
            ];

            if column_states.into_iter().all(|state| state == State::Cross) {
                return Some(GridResult::CrossWon)
            }

            if column_states.into_iter().all(|state| state == State::Naught) {
                return Some(GridResult::NaughtWon)
            }
        }

        None
    }

    fn check_diagonals_for_winner(&self) -> Option<GridResult> {
        let left_to_right = [
            self.squares[0][0].state, self.squares[1][1].state, self.squares[2][2].state
        ];
        let right_to_left = [
            self.squares[0][2].state, self.squares[1][1].state, self.squares[2][0].state
        ];

        if left_to_right.into_iter().all(|state| state == State::Cross)
            || right_to_left.into_iter().all(|state| state == State::Cross) {
            return Some(GridResult::CrossWon)
        }

        if left_to_right.into_iter().all(|state| state == State::Naught)
            || right_to_left.into_iter().all(|state| state == State::Naught) {
            return Some(GridResult::NaughtWon)
        }

        None
    }

    fn check_for_completed_grid(&self) -> Option<GridResult> {
        for row in self.squares.iter() {
            for square in row.iter() {
                if matches!(square.state, State::Empty) {
                    return None;
                }
            }
        }

        Some(GridResult::Complete)
    }

    fn is_taken(&self, x_coordinate : usize, y_coordinate: usize) -> bool {
        self.squares[y_coordinate][x_coordinate].state != State::Empty
    }
}

impl GridHandler for Grid {
    fn is_complete(&self) -> GridResult {
        if let Some(result) = self.check_rows_for_winner() {
            return result
        }

        if let Some(result) = self.check_columns_for_winner() {
            return result
        }

        if let Some(result) = self.check_diagonals_for_winner() {
            return result
        }

        if let Some(result) = self.check_for_completed_grid() {
            return result
        }

        GridResult::Incomplete
    }

    fn make_move(&mut self, x_coordinate : usize, y_coordinate: usize, mark: Mark) -> Result<(), String> {
        if self.squares.get(y_coordinate).and_then(|row| row.get(x_coordinate)).is_none() {
            return Err("Coordinates are out of range".to_string())
        }

        if self.is_taken(x_coordinate, y_coordinate) {
            return Err("Square is already taken".to_string())
        }

        let state = match mark {
            Mark::Cross => State::Cross,
            Mark::Naught => State::Naught
        };

        let square = self.squares
            .get_mut(y_coordinate)
            .and_then(|row| row.get_mut(x_coordinate))
            .ok_or_else(|| "Coordinates are out of range".to_string())?;

        square.state = state;
        Ok(())
    }

    fn available_spaces(&self) -> Vec<(usize, usize)> {
        let mut available_spaces = vec![];

        for y in 0..3 {
            for x in 0..3 {
                if self.squares[y][x].state == State::Empty {
                    available_spaces.push((x, y))
                }
            }
        }

        available_spaces
    }
}

pub struct Square {
    pub state: State,
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mark {
    Cross,
    Naught
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Empty,
    Cross,
    Naught,
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GridResult {
    CrossWon,
    NaughtWon,
    Complete,
    Incomplete,
}


#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(grid = {
        Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Cross }, Square { state: State::Cross }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Cross }, Square { state: State::Cross }, Square { state: State::Cross }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Cross }, Square { state: State::Cross }, Square { state: State::Cross }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Naught }, Square { state: State::Naught }, Square { state: State::Naught }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Naught }, Square { state: State::Naught }, Square { state: State::Naught }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Naught }, Square { state: State::Naught }, Square { state: State::Naught }],
            ]
        }
    }, result = {
        GridResult::CrossWon,
        GridResult::CrossWon,
        GridResult::CrossWon,
        GridResult::NaughtWon,
        GridResult::NaughtWon,
        GridResult::NaughtWon,
    })]
    fn test_grid_handler_check_rows_for_winner(grid: Grid, result: GridResult) {
        assert_eq!(grid.check_rows_for_winner().unwrap(), result);
    }

    #[parameterized(grid = {
        Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Cross }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Cross }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Cross }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Cross }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Cross }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Cross }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Naught }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Naught }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Naught }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Naught }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Naught }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Naught }],
            ]
        }
    }, result = {
        GridResult::CrossWon,
        GridResult::CrossWon,
        GridResult::CrossWon,
        GridResult::NaughtWon,
        GridResult::NaughtWon,
        GridResult::NaughtWon,
    })]
    fn test_grid_handler_check_columns_for_winner(grid: Grid, result: GridResult) {
        assert_eq!(grid.check_columns_for_winner().unwrap(), result);
    }

    #[parameterized(grid = {
        Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Cross }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Cross }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Cross }],
                [Square { state: State::Empty }, Square { state: State::Cross }, Square { state: State::Empty }],
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Naught }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Naught }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Naught }],
                [Square { state: State::Empty }, Square { state: State::Naught }, Square { state: State::Empty }],
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
    }, result = {
        GridResult::CrossWon,
        GridResult::CrossWon,
        GridResult::NaughtWon,
        GridResult::NaughtWon,
    })]
    fn test_grid_handler_check_diagonals_for_winner(grid: Grid, result: GridResult) {
        assert_eq!(grid.check_diagonals_for_winner().unwrap(), result);
    }

    #[parameterized(grid = {
        Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Naught }, Square { state: State::Cross }],
                [Square { state: State::Naught }, Square { state: State::Naught }, Square { state: State::Cross }],
                [Square { state: State::Cross }, Square { state: State::Cross }, Square { state: State::Naught }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Cross }, Square { state: State::Cross }],
                [Square { state: State::Naught }, Square { state: State::Naught }, Square { state: State::Cross }],
                [Square { state: State::Naught }, Square { state: State::Cross }, Square { state: State::Naught }],
            ]
        },
    }, result = {
        GridResult::Complete,
        GridResult::Complete,
    })]
    fn test_grid_handler_check_for_completed_grid_when_full(grid: Grid, result: GridResult) {
        assert_eq!(grid.check_for_completed_grid().unwrap(), result);
    }

    #[parameterized(grid = {
        Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Naught }, Square { state: State::Cross }],
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Cross }],
                [Square { state: State::Cross }, Square { state: State::Cross }, Square { state: State::Naught }],
            ]
        },
        Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        },
    })]
    fn test_grid_handler_check_for_completed_grid_when_not_full(grid: Grid) {
        assert_eq!(grid.check_for_completed_grid(), None);
    }

    #[test]
    fn test_grid_handler_check_rows_for_winner_when_no_winner() {
        let grid = Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Naught }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        };

        assert_eq!(grid.check_rows_for_winner(), None);
    }

    #[test]
    fn test_grid_handler_check_columns_for_winner_when_no_winner() {
        let grid = Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Naught }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        };

        assert_eq!(grid.check_columns_for_winner(), None);
    }

    #[test]
    fn test_grid_handler_check_diagonals_for_winner_when_no_winner() {
        let grid = Grid {
            squares: [
                [Square { state: State::Cross }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Naught }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        };

        assert_eq!(grid.check_diagonals_for_winner(), None);
    }

    #[test]
    fn test_grid_handler_make_move() {
        let mut grid = Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        };

        grid.make_move(1, 1, Mark::Cross);
        grid.make_move(0, 2, Mark::Naught);

        assert_eq!(grid.squares[1][1].state, State::Cross);
        assert_eq!(grid.squares[2][0].state, State::Naught)
    }

    #[test]
    fn test_grid_handler_make_move_coordinates_out_of_range() {
        let mut grid = Grid {
            squares: [
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        };

        assert_eq!(
            grid.make_move(3, 0, Mark::Cross).unwrap_err(),
            "Coordinates are out of range"
        );
        assert_eq!(
            grid.make_move(0, 3, Mark::Cross).unwrap_err(),
            "Coordinates are out of range"
        );
    }

    #[parameterized(existing_state = {
        State::Cross,
        State::Naught,
    }, mark = {
        Mark::Naught,
        Mark::Cross,
    })]
    fn test_grid_handler_make_move_square_already_taken(existing_state: State, mark: Mark) {
        let mut grid = Grid {
            squares: [
                [Square { state: existing_state }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
                [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            ]
        };

        assert_eq!(
            grid.make_move(0, 0, mark).unwrap_err(),
            "Square is already taken"
        );
        assert_eq!(grid.squares[0][0].state, existing_state);
    }
}
