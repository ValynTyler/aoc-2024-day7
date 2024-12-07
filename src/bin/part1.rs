use aoc_2024_day7::{concat::Concat, puzzle_input::PuzzleInput};

fn main() {
    println!("hello from part 1");

    let input_string = include_str!("../../input/example.txt");
    let input = PuzzleInput::from(input_string);

    let mut sum = 0;
    for item in input.0 {
        let possible = item.is_possible(vec![
            |a, b| a + b,
            |a, b| a * b,
        ]);

        println!("{:?} {:#?}", item.values, possible);
        if possible { sum += item.expect };
    }
    println!("{}", sum);
}
