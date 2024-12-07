use aoc_2024_day7::puzzle_input::PuzzleInput;

fn main() {
    println!("hello from part 1");

    let input_string = include_str!("../../input/example.txt");
    let input = PuzzleInput::from(input_string);

    println!("{:#?}", input);
}
