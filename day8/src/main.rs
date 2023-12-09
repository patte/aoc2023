use std::collections::HashMap;

/*
https://adventofcode.com/2023/day/8
*/
#[allow(dead_code)]
static EXAMPLE_PART1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
// answer: 2

#[allow(dead_code)]
static EXAMPLE_PART2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
// answer: 6

// this can be solved with a hash map and made fast with a tree
// for now only the hash map is implemented, as it solves the example in 0.01s
//
// nice tree library:
// https://github.com/saschagrunert/indextree
// 🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let mut lines = input.lines();

    // LRLLR (round robin)
    let directions = lines.next().unwrap();
    println!("directions: {}", directions);

    let mut map_entries = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        // parse line eg.:
        // AAA = (BBB, CCC)
        let (node, (edge_left, edge_right)) = line
            .split_once(" = (")
            .map(|(n, e)| {
                (
                    n.to_string(),
                    e[..8] // remove end bracket
                        .split_once(", ")
                        .map(|(l, r)| (l.to_string(), r.to_string()))
                        .unwrap(),
                )
            })
            .unwrap();

        // store in map
        map_entries.insert(node, (edge_left, edge_right));
    }
    //println!("{:#?}", map_entries);

    (directions.to_string(), map_entries)
}

fn solve_labirynth(
    directions: &str,
    map_entries: &HashMap<String, (String, String)>,
    start: &str,
    end: &str,
    start_direction_index: u64,
    print_steps: bool,
) -> (String, u64) {
    let mut current_node = start;
    let mut step_count: u64 = 0;

    for direction in directions.chars().cycle().skip(
        // modulo is needed because skip wants a usize and start_direction_index is u64
        (start_direction_index % directions.len() as u64)
            .try_into()
            .unwrap(),
    ) {
        match map_entries.get(current_node) {
            Some((edge_left, edge_right)) => {
                if print_steps {
                    if step_count < 4 {
                        print!("{}-[{}]->", current_node, direction);
                    } else if step_count == 4 {
                        print!("...");
                    }
                }
                match direction {
                    'L' => {
                        current_node = edge_left;
                    }
                    'R' => {
                        current_node = edge_right;
                    }
                    _ => {
                        panic!("direction not found {}", direction);
                    }
                }
                step_count += 1;
                if print_steps && step_count < 5 {
                    println!("{} {}", current_node, start_direction_index + step_count);
                }
            }
            None => {
                panic!("node not found {}", current_node);
            }
        }
        match end.len() {
            1 => {
                if current_node.ends_with(end) {
                    break;
                }
            }
            _ => {
                if current_node == end {
                    break;
                }
            }
        }
    }
    if print_steps && step_count >= 5 {
        println!(
            "    ->{} {}",
            current_node,
            start_direction_index + step_count
        );
    }
    (current_node.to_string(), step_count)
}

// get least common multiple of two numbers
fn lcm(a: u64, b: u64) -> u64 {
    let greater = a.max(b);
    let smaller = a.min(b);
    for i in (greater..=a * b).step_by(greater.try_into().unwrap()) {
        if i % smaller == 0 {
            return i;
        }
    }
    panic!("no lcm found");
}

#[derive(Debug)]
struct Ghost {
    current_node: String,
    step_count: u64,
    loop_step_count: u64,
}

fn main() {
    println!("✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫");
    println!("Hello, adventofcode.com/2023/day/8 from rust!");

    let args = std::env::args().collect::<Vec<String>>();
    let part = if args.len() > 1 {
        args[1].parse::<u64>().unwrap()
    } else {
        1
    };
    println!("--- Part {} ---", if part == 1 { "One" } else { "Two" });

    #[allow(unused_variables)]
    let input = if part == 1 {
        EXAMPLE_PART1
    } else {
        EXAMPLE_PART2
    };
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (directions, map_entries) = parse_input(&input);

    if part == 1 {
        let (_, step_count) = solve_labirynth(&directions, &map_entries, "AAA", "ZZZ", 0, true);
        println!("steps: {}", step_count);
    } else {
        let mut ghosts = map_entries
            .iter()
            .filter(|(n, _)| n.ends_with("A"))
            .map(|(n, _)| Ghost {
                current_node: n.to_string(),
                step_count: 0,
                loop_step_count: 0,
            })
            .collect::<Vec<Ghost>>();

        let mut max_step_count = ghosts.iter().map(|c| c.step_count).max().unwrap();
        while ghosts
            .iter()
            .any(|c| c.step_count != max_step_count && c.loop_step_count == 0)
            || ghosts.iter().all(|c| c.step_count == 0)
        {
            for ghost in ghosts.iter_mut() {
                // println!("ghost: {:?} {}", ghost, max_step_count - ghost.step_count);
                if ghost.step_count >= max_step_count && ghost.step_count > 0 {
                    println!("skip: {:?}", ghost);
                    continue;
                }
                let (new_node, additional_steps) = solve_labirynth(
                    &directions,
                    &map_entries,
                    &ghost.current_node,
                    "Z",
                    ghost.step_count,
                    true,
                );
                // if we detect a loop, we don't add step_count
                if ghost.current_node == new_node {
                    println!("         loop!!!");
                    ghost.loop_step_count = additional_steps;
                    continue;
                }
                ghost.step_count += additional_steps;
                ghost.current_node = new_node;

                max_step_count = max_step_count.max(ghost.step_count);
            }
            println!("ghosts: {:?}", ghosts);
            println!("max_step_count: {}", max_step_count);
        }

        let first_step_counts = ghosts.first().unwrap().step_count;
        if ghosts.iter().all(|c| c.step_count == first_step_counts) {
            println!("steps: {}", first_step_counts);
            return;
        } else {
            // we stopped because all ghosts with step_count < max_step_count are looping
            // find least common multiple of all step counts
            let mut all_step_counts = ghosts.iter().map(|c| c.step_count).collect::<Vec<u64>>();
            all_step_counts.dedup();
            all_step_counts =
                all_step_counts
                    .iter()
                    .fold(vec![all_step_counts[0]], |mut acc, &c| {
                        acc.push(lcm(acc.last().unwrap().to_owned(), c));
                        acc
                    });
            let max_step_count = all_step_counts.iter().max().unwrap();
            println!("steps: {:#?}", max_step_count);
        }
    }
}
