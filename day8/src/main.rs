use std::collections::HashMap;

/*
https://adventofcode.com/2023/day/8
*/
#[allow(dead_code)]
static EXAMPLE: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
// answer: 2

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

    //let input = EXAMPLE;
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (directions, map_entries) = parse_input(&input);

    let mut current_node = "AAA";
    let mut step_count = 0;
    for direction in directions.chars().cycle() {
        match map_entries.get(current_node) {
            Some((edge_left, edge_right)) => {
                if step_count < 4 {
                    print!("{}-[{}]->", current_node, direction);
                } else if step_count == 4 {
                    println!("...");
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
                if step_count < 5 {
                    println!("{} {}", current_node, step_count);
                }
            }
            None => {
                panic!("node not found {}", current_node);
            }
        }
        if current_node == "ZZZ" {
            break;
        }
    }
    println!("{} steps", step_count);
}
