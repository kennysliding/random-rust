mod day_1;
mod day_2;
mod day_3;

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

    println!(
        "Day 2/Part 1: {}",
        day_2::handcraft::SolutionDay2::solve_1("src/day_2/input.txt")
    );

    println!(
        "Day 2/Part 2: {}",
        day_2::handcraft::SolutionDay2::solve_2("src/day_2/input.txt")
    );

    println!(
        "Day 3/Part 1: {}",
        day_3::handcraft::SolutionDay3::solve_1("src/day_3/input.txt")
    );

    println!(
        "Day 3/Part 2: {}",
        day_3::handcraft::SolutionDay3::solve_2("src/day_3/input.txt")
    );
}
