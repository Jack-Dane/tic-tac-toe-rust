use super::grid::{GridHandler, Mark};
use rand::{random_range};

pub trait Opponent {
    fn make_move(&self, grid: &mut impl GridHandler, mark: Mark) -> Result<(), String>;
}

pub struct RandomChoiceOpponent {}

impl Opponent for RandomChoiceOpponent {
    fn make_move(&self, grid: &mut impl GridHandler, mark: Mark) -> Result<(), String> {
        let available_spaces = grid.available_spaces();

        let choice = available_spaces[random_range(0..available_spaces.len())];

        grid.make_move(choice.0, choice.1, mark)
    }
}


#[cfg(test)]
mod tests {
    use std::fmt;
    use crate::grid::GridResult;
    use super::*;

    const AVAILABLE_SPACE: [(usize, usize); 3] = [(1, 2), (3, 3), (4, 5)];

    struct TestGrid {
        move_made: Option<(usize, usize, Mark)>
    }

    impl fmt::Display for TestGrid {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }

    impl GridHandler for TestGrid {
        fn is_complete(&self) -> GridResult {
            GridResult::Incomplete
        }

        fn make_move(&mut self, x_coordinate: usize, y_coordinate: usize, state: Mark) -> Result<(), String> {
            self.move_made = Some((x_coordinate, y_coordinate, state));

            Ok(())
        }

        fn available_spaces(&self) -> Vec<(usize, usize)> {
            AVAILABLE_SPACE.to_vec()
        }
    }

    #[test]
    fn test_random_choice_opponent_make_move_cross() {
        let opponent = RandomChoiceOpponent {};
        let mut grid = TestGrid {
            move_made: None
        };

        let result = opponent.make_move(&mut grid, Mark::Cross);

        assert_eq!(result, Ok(()));
        let move_made = grid.move_made.unwrap();
        assert!(AVAILABLE_SPACE.contains(&(move_made.0, move_made.1)));
        assert_eq!(Mark::Cross, move_made.2);
    }

    #[test]
    fn test_random_choice_opponent_make_move_naught() {
        let opponent = RandomChoiceOpponent {};
        let mut grid = TestGrid {
            move_made: None
        };

        let result = opponent.make_move(&mut grid, Mark::Naught);

        assert_eq!(result, Ok(()));
        let move_made = grid.move_made.unwrap();
        assert!(AVAILABLE_SPACE.contains(&(move_made.0, move_made.1)));
        assert_eq!(Mark::Naught, move_made.2);
    }

}
