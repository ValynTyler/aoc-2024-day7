pub struct Equation {
    result: i32,
    values: Vec<i32>,
}

impl From::<&str> for Equation {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(':').unwrap(); // TODO
        let result: i32 = left
            .trim()
            .parse()
            .unwrap(); // TODO

        let values: Vec<_> = right
            .trim()
            .split_whitespace()
            .map(|token| token.parse::<i32>().unwrap()) // TODO
            .collect();

        Self { result, values }
    }
}
