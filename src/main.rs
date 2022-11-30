use lib::day_one;
use lib::day_three;
use lib::day_two;

fn main() {
    day_one();
    println!();
    day_two();
    println!();
    day_three();
    println!();
}

fn day_one() {
    let day_one_input = std::fs::read_to_string("src/input/day_one.txt")
        .expect("Something went wrong with fetch day one input!");

    println!(
        "The answer for day one, part one is: {}",
        day_one::process_paranthesis(day_one_input.clone())
    );
    println!(
        "The answer for day one, part two is: {}",
        day_one::basement_entered(day_one_input)
    );
}

fn day_two() {
    let day_two_input = std::fs::read_to_string("src/input/day_two.txt")
        .expect("Something went wrong with fetch day one input!");

    println!(
        "The answer for day two, part one is: {}",
        day_two::process_presents_wrapper(day_two_input.clone())
    );

    println!(
        "The answer for day two, part two is: {}",
        day_two::process_presents_ribbon(day_two_input)
    );
}

fn day_three() {
    let day_three_input = std::fs::read_to_string("src/input/day_three.txt")
        .expect("Something went wrong with fetch day one input!");

    println!(
        "The answer for day two, part one is: {}",
        day_three::process_moves_santa_only(day_three_input.clone())
    );

    println!(
        "The answer for day two, part two is: {}",
        day_three::process_moves_santa_robo_santa(day_three_input)
    );
}
