use super::grid::{Grid, GridHandler, GridResult, Mark, Square, State};
use super::opponent::{Opponent, RandomChoiceOpponent};

const PLAYER_MARK: Mark = Mark::Naught;
const OPPONENT_MARK: Mark = Mark::Cross;

fn convert_input_to_coordinates(input: &str) -> Result<(usize, usize), String> {
    let first_input = input.get(0..1).ok_or("Incorrect input format")?;
    let second_input = input.get(1..2).ok_or("Incorrect input format")?;

    let first_input_int: usize = first_input.parse().map_err(|_| "First character is not a number")?;
    let second_input_int: usize = second_input.parse().map_err(|_| "Second character is not a number")?;

    Ok((first_input_int, second_input_int))
}

pub fn run_game() {
    let mut game_state = start_game();

    while !game_state.is_complete() {
        game_state.display_grid();
        println!("Choose a square to place your mark (row and column):");

        let mut input = String::new();
        loop {
            input.clear();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            let inputs = match convert_input_to_coordinates(input) {
                Ok(inputs) => inputs,
                Err(msg) => panic!("{}", msg)
            };

            match game_state.make_player_move(inputs.0, inputs.1) {
                Ok(()) => break,
                Err(e) => {
                    println!("{}", e);
                    println!("Please try again...");
                }
            };
        }

        println!("Opponent making move...");
        match game_state.make_opponent_move() {
            Ok(()) => (),
            Err(e) => {
                println!("{}", e);
                panic!("Failure opponent choosing a space")
            }
        };
    }
}

fn start_game() -> impl GameStateHandler {
    let grid: Grid = Grid {
        squares: [
            [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }],
            [Square { state: State::Empty }, Square { state: State::Empty }, Square { state: State::Empty }]
        ]
    };

    GameState {
        grid,
        opponent: RandomChoiceOpponent {}
    }
}

struct GameState<G: GridHandler, O: Opponent> {
    grid: G,
    opponent: O,
}

pub trait GameStateHandler {
    fn make_player_move(&mut self, x_coordinate: usize, y_coordinate: usize) -> Result<(), String>;
    fn make_opponent_move(&mut self) -> Result<(), String>;
    fn is_complete(&self) -> bool;
    fn display_grid(&self);
}

impl<G: GridHandler, O: Opponent> GameStateHandler for GameState<G, O> {

    fn make_player_move(&mut self, x_coordinate: usize, y_coordinate: usize) -> Result<(), String> {
        self.grid.make_move(x_coordinate, y_coordinate, PLAYER_MARK)
    }

    fn make_opponent_move(&mut self) -> Result<(), String> {
        self.opponent.make_move(&mut self.grid, OPPONENT_MARK)
    }

    fn is_complete(&self) -> bool {
        match self.grid.is_complete() {
            GridResult::CrossWon => {true}
            GridResult::NaughtWon => {true}
            GridResult::Complete => {true}
            GridResult::Incomplete => {false}
        }
    }

    fn display_grid(&self) {
        print!("{}", &self.grid);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::fmt;
    use std::fmt::Formatter;
    use super::*;
    use parameterized::parameterized;

    struct FakeGrid {
        result: GridResult,
        move_made: Option<(usize, usize, Mark)>
    }

    impl GridHandler for FakeGrid {
        fn is_complete(&self) -> GridResult {
            self.result
        }

        fn make_move(&mut self, x_coordinate: usize, y_coordinate: usize, state: Mark) -> Result<(), String> {
            self.move_made = Some((x_coordinate, y_coordinate, state));

            Ok(())
        }

        fn available_spaces(&self) -> Vec<(usize, usize)> {
            vec![(0, 0)]
        }
    }

    struct FakeOpponent {
        move_made: Cell<Option<Mark>>
    }

    impl Opponent for FakeOpponent {
        fn make_move(&self, grid: &mut impl GridHandler, mark: Mark) -> Result<(), String> {
            self.move_made.set(Some(mark));

            Ok(())
        }
    }

    impl fmt::Display for FakeGrid {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "FakeGrid({:?})", self.result)
        }
    }

    #[parameterized(grid_result = {
        GridResult::CrossWon,
        GridResult::NaughtWon,
        GridResult::Complete,
        GridResult::Incomplete,
    }, expected = {
        true,
        true,
        true,
        false,
    })]
    fn test_game_state_is_complete(grid_result: GridResult, expected: bool) {
        let game_state = GameState {
            grid: FakeGrid {
                result: grid_result,
                move_made: None,
            },
            opponent: FakeOpponent {
                move_made: Cell::new(None)
            }
        };

        assert_eq!(game_state.is_complete(), expected);
    }

    #[test]
    fn test_game_state_make_opponent_move() {
        let mut game_state = GameState {
            grid: FakeGrid {
                result: GridResult::Incomplete,
                move_made: None,
            },
            opponent: FakeOpponent {
                move_made: Cell::new(None)
            }
        };

        let result = game_state.make_opponent_move();

        assert_eq!(Ok(()), result);
        assert_eq!(Some(Mark::Cross), game_state.opponent.move_made.take());
    }

    #[test]
    fn test_game_state_make_player_move() {
        let mut game_state = GameState {
            grid: FakeGrid {
                result: GridResult::Incomplete,
                move_made: None,
            },
            opponent: FakeOpponent {
                move_made: Cell::new(None)
            }
        };

        let result = game_state.make_player_move(1, 2);

        assert_eq!(Some((1, 2, Mark::Naught)), game_state.grid.move_made)
    }
}
