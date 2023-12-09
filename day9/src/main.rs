/*
https://adventofcode.com/2023/day/9
*/
#[allow(dead_code)]
static EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
// answer:
// 0 3 6 9 12 15 18
// 1 3 6 10 15 21 28
// 10 13 16 21 30 45 68
// => 114

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn get_diffs(history: &Vec<i64>) -> Vec<i64> {
    history
        .iter()
        .enumerate()
        .filter(|(i, _)| *i > 0) // skip first entry
        .map(|(i, n)| n - history[i - 1]) // diff to previous entry
        .collect::<Vec<i64>>()
}

fn main() {
    println!("✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫");
    println!("Hello, adventofcode.com/2023/day/9 from rust!");

    let args = std::env::args().collect::<Vec<String>>();
    let part = if args.len() > 1 {
        args[1].parse::<i64>().unwrap()
    } else {
        1
    };
    println!("--- Part {} ---", if part == 1 { "One" } else { "Two" });

    #[allow(unused_variables)]
    //let input = EXAMPLE;
    let input = std::fs::read_to_string("input.txt").unwrap();

    let histories = parse_input(&input);
    println!("histories:");
    for history in histories.iter().take(4) {
        println!("  {:?}", history);
    }
    if histories.len() > 4 {
        println!("  ...");
    }

    let mut next_entries = Vec::new();
    let mut previous_entries = Vec::new();
    for history in histories.iter() {
        let mut last_diff = history.clone();
        let mut last_entry_of_diffs = vec![last_diff.iter().last().unwrap().clone()];
        let mut first_entry_of_diffs = vec![last_diff.iter().next().unwrap().clone()];

        loop {
            let diff = get_diffs(&last_diff).to_owned();
            if diff.iter().all(|d| d == &0) {
                break;
            }
            last_entry_of_diffs.push(diff.iter().last().unwrap().clone());
            first_entry_of_diffs.push(diff.iter().next().unwrap().clone());
            last_diff = diff;
        }
        // println!("last_diff: {:?}", last_diff);
        // println!("last_entry_of_diffs: {:?}", last_entry_of_diffs);
        // println!("first_entry_of_diffs: {:?}", first_entry_of_diffs);
        println!("⏅ levels: {:?}", last_entry_of_diffs.len());

        let next_entry = last_entry_of_diffs.iter().sum::<i64>();
        next_entries.push(next_entry);

        let previous_entry = first_entry_of_diffs
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, n)| if i == 0 { *n } else { n - acc });
        previous_entries.push(previous_entry);
    }
    let answer = match part {
        2 => previous_entries.iter().sum::<i64>(),
        _ => next_entries.iter().sum::<i64>(),
    };
    println!("answer: {:?}", answer);
}
