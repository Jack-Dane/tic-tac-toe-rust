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
