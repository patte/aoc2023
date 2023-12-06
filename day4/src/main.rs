#[allow(dead_code)]
static EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn main() {
    println!("Hello, adventofcode.com/2023/day/4 from rust!");

    let contents = EXAMPLE;
    //let contents = std::fs::read_to_string("input.txt").unwrap();
    let (points, num_cards) = get_points_and_num_cards(&contents);
    println!("----------------------");
    println!("points: {}", points);
    println!("num_cards: {}", num_cards);
}

fn get_points_and_num_cards(contents: &str) -> (u32, u32) {
    assert_eq!((0.5) as u32, 0);
    assert_eq!(2u32.pow((0 - 1) as u32), 0);
    assert_eq!(2u32.pow((1 - 1) as u32), 1);
    assert_eq!(2u32.pow((2 - 1) as u32), 2);
    assert_eq!(2u32.pow((3 - 1) as u32), 4);

    let first_line = contents.lines().next().unwrap();
    if first_line.len() == 0 {
        panic!("first line is empty");
    }
    println!("first_line:\n{}", first_line);
    let separator_index = first_line
        .char_indices()
        .find(|(_, c)| *c == '|')
        .unwrap()
        .0;
    let card_end_index = first_line
        .char_indices()
        .find(|(_, c)| *c == ':')
        .unwrap()
        .0
        + 2; // +2 to include the ': '
    let num_winning_numbers = (separator_index - card_end_index) / 3; // 3: space + 2 digits

    // figure out which of the numbers you have appear in the list of winning numbers.
    // The first match makes the card worth one point and each match after the first
    // doubles the point value of that card.
    let mut points: u32 = 0;

    // part two
    let mut won_cards_counters: Vec<u32> = vec![0; contents.lines().count()];

    // go through each line
    for (index, line) in contents.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let parsed_numbers: Vec<u32> = line[card_end_index..]
            .split(' ')
            .filter(|s| s.len() > 0 && s != &"|" && s != &" ")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let winning_numbers: Vec<u32> = parsed_numbers[0..num_winning_numbers].to_vec();
        let numbers_we_have: Vec<u32> = parsed_numbers[num_winning_numbers..].to_vec();

        let winning_numbers_we_have: Vec<u32> = winning_numbers
            .iter()
            .filter(|n| numbers_we_have.contains(n))
            .map(|n| *n)
            .collect();
        let num_winning_numbers_we_have = winning_numbers_we_have.len() as u32;

        // part one
        // doubling with 2^x
        // see asserts above regarding u32
        let points_this_line = 2u32.pow((num_winning_numbers_we_have - 1) as u32);
        points += points_this_line;

        // part two
        // increase count for following cards by amount of cards we have at current index
        for i in index + 1..(index + 1 + num_winning_numbers_we_have as usize) {
            won_cards_counters[i] += won_cards_counters[index] + 1;
        }

        if index == 0 {
            println!("winning_numbers: {:?}", winning_numbers);
            println!("numbers_we_have: {:?}", numbers_we_have);
            println!("winning_numbers_we_have: {:?}", winning_numbers_we_have);
            println!("points_this_line: {}", points_this_line);
            println!("won_cards_counters: {:?}", won_cards_counters);
        }
    }

    let num_cards = contents.lines().count() as u32 + won_cards_counters.iter().sum::<u32>();

    return (points, num_cards);
}
