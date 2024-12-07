use crate::equation::Equation;

#[derive(Debug)]
pub struct PuzzleInput(pub Vec<Equation>);

impl From::<&str> for PuzzleInput {
    fn from(value: &str) -> Self {
        Self(value
            .trim()
            .lines()
            .map(|line| Equation::from(line))
            .collect::<Vec<_>>()
        )
    }
}
