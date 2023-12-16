#[allow(dead_code)]
static EXAMPLE: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

/*
rn=1 becomes 30.
cm- becomes 253.
qp=3 becomes 97.
cm=2 becomes 47.
qp- becomes 14.
pc=4 becomes 180.
ot=9 becomes 9.
ab=5 becomes 197.
pc- becomes 48.
pc=6 becomes 214.
ot=7 becomes 231.

In this example, the sum of these results is 1320.
*/

fn hash_algorithm(input: &str) -> u32 {
    let mut current_value: u32 = 0;
    for c in input.chars() {
        let ascii = c as u32;
        current_value += ascii;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}
fn main() {
    //let input = "HASH";
    //let input = EXAMPLE;
    let input = include_str!("../input.txt");
    //println!("input:\n{}", input);

    let part = 1;
    println!("part: {}", part);

    let steps = input.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
    //println!("steps: {:#?}", steps);

    let mut sum: u32 = 0;
    for step in steps {
        let hash = hash_algorithm(step);
        println!("{}: {}", step, hash);
        sum += hash;
    }
    println!("sum: {}", sum);
}
