mod day_1;

fn main() {
    println!("Advent of Code 2025!");

    println!(
        "Day 1/Part 1: {}",
        day_1::handcraft::SolutionDay1::solve_1("src/day_1/input.txt")
    );
    println!(
        "Day 1/Part 2: {}",
        day_1::handcraft::SolutionDay1::solve_2("src/day_1/input.txt")
    );
}
