use aoc_2024_day7::puzzle_input::PuzzleInput;

fn main() {
    println!("hello from part 1");

    let input_string = include_str!("../../input/numbers.txt");
    let input = PuzzleInput::from(input_string);

    let mut sum = 0;
    for item in input.0 {
        let possible = item.is_possible();
        println!("{:#?}", possible);
        if possible { sum += item.expect };
    }
    println!("{}", sum);
}
