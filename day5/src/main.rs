mod day5;
use day5::*;

// first param: [part]
// second param: [--example]
fn main() {
    println!("Hello, adventofcode.com/2023/day/5 from rust!");

    let args = std::env::args().collect::<Vec<String>>();
    let part = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(1)
    } else {
        1
    };
    let use_example = if args.len() > 2 {
        "--example" == args[2].parse::<String>().unwrap_or("".to_string())
    } else {
        false
    };
    println!("--- Part {} ---", if part == 1 { "One" } else { "Two" });

    let almanac = if use_example {
        EXAMPLE.to_string()
    } else {
        std::fs::read_to_string("input.txt").unwrap()
    };

    // print first 4 lines
    println!(
        "almanac:\n{}",
        almanac.lines().take(4).collect::<Vec<&str>>().join("\n")
    );

    let answer = get_answer_for_almanac(&almanac, Some(part));
    println!("answer: {:?}", answer);
}
