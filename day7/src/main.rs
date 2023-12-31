/*
https://adventofcode.com/2023/day/7
*/
#[allow(dead_code)]
static EXAMPLE: &str = "\
32T3K 765
T55J5 684
KK767 28
KTJJT 220
QQQJA 483";
//AAAAA 6
//AA8AA 5
//23332 4
//TTT98 3
//23432 2
//A23A4 1
//23456 0";

static CARDS_PART1: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];
static CARDS_PART2: [&str; 13] = [
    "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
];

fn get_rank_for_card(c: char, cards: &[&str]) -> usize {
    cards.iter().position(|&x| x == c.to_string()).unwrap()
    //+ cards[0].parse::<usize>().unwrap() // move so that 2 has rank 2
}

fn get_card_for_rank(r: usize, cards: &[&str]) -> String {
    cards[r].to_string()
}

static TYPS: [&str; 7] = [
    "High Card",
    "One Pair",
    "Two Pairs",
    "Three of a Kind",
    "Full House",
    "Four of a Kind",
    "Five of a Kind",
];

fn get_rank_for_typ(typ: &str) -> usize {
    TYPS.iter().position(|&x| x == typ).unwrap()
}

#[derive(Debug, Clone)]
struct Hand {
    #[allow(dead_code)]
    cards: String,
    card_ranks: Vec<usize>,
    #[allow(dead_code)]
    typ: String,
    type_rank: u64,
    rank: u64,
    bid: u64,
}

fn main() {
    println!("Hello, adventofcode.com/2023/day/7 from rust!");
    let args = std::env::args().collect::<Vec<String>>();
    let part = if args.len() > 1 {
        args[1].parse::<u64>().unwrap()
    } else {
        1
    };
    println!("--- Part {} ---", if part == 1 { "One" } else { "Two" });

    #[allow(non_snake_case)]
    let CARDS = if part == 1 {
        &CARDS_PART1
    } else {
        &CARDS_PART2
    };

    //let input = EXAMPLE;
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let mut line_parts = line.split_whitespace();
        let cards = line_parts.next().unwrap().to_string();

        let card_ranks = cards
            .chars()
            .map(|c| get_rank_for_card(c, CARDS))
            .collect::<Vec<usize>>();

        let mut card_ranks_sorted = card_ranks.clone();
        card_ranks_sorted.sort();

        let cards_sorted = card_ranks_sorted
            .iter()
            .map(|&r| get_card_for_rank(r, CARDS))
            .collect::<String>();

        let mut card_groups: Vec<(String, usize)> = Vec::new();

        let mut last_card_group = ("".to_string(), 0);
        for c in cards_sorted.chars() {
            if last_card_group.0 == c.to_string() {
                last_card_group.1 += 1;
            } else {
                if last_card_group.0 != "".to_string() {
                    card_groups.push(last_card_group);
                }
                last_card_group = (c.to_string(), 1);
            }
        }
        card_groups.push(last_card_group);

        card_groups.sort_by(|a, b| b.1.cmp(&a.1));

        // part2
        if part == 2 {
            let contains_joker = cards.contains("J");
            if contains_joker && card_groups.len() > 1 {
                let mut joker_group_index = 0;
                let mut joker_group_counter = 0;
                for (i, g) in card_groups.iter().enumerate() {
                    if g.0 == "J" {
                        joker_group_index = i;
                        joker_group_counter = g.1;
                        break;
                    }
                }
                let apply_to_group_index = if joker_group_index == 0 { 1 } else { 0 };
                card_groups[apply_to_group_index].1 += joker_group_counter;
                card_groups.remove(joker_group_index);
            }
        }

        let card_groups_len = card_groups.len();

        let mut typ = "";
        if card_groups_len == 1 {
            typ = "Five of a Kind";
        } else if card_groups_len == 2 {
            if card_groups[0].1 == 4 {
                typ = "Four of a Kind";
            } else {
                typ = "Full House";
            }
        } else if card_groups_len == 3 {
            if card_groups[0].1 == 3 {
                typ = "Three of a Kind";
            }
            if card_groups[0].1 == 2 {
                typ = "Two Pairs";
            }
        } else if card_groups_len == 4 {
            typ = "One Pair";
        } else if card_groups_len == 5 {
            typ = "High Card";
        }

        let type_rank = get_rank_for_typ(&typ) as u64;

        let hand = Hand {
            cards,
            card_ranks,
            typ: typ.to_string(),
            type_rank,
            rank: 0,
            bid: line_parts.next().unwrap().parse::<u64>().unwrap(),
        };
        hands.push(hand);
    }

    // sort hands by type_rank, then by card_ranks
    hands.sort_by(|a, b| {
        if a.type_rank == b.type_rank {
            for i in 0..5 {
                if a.card_ranks[i] != b.card_ranks[i] {
                    return b.card_ranks[i].cmp(&a.card_ranks[i]);
                }
            }
            return std::cmp::Ordering::Equal;
        }
        return b.type_rank.cmp(&a.type_rank);
    });

    // set rank
    let hands_len = hands.len();
    for i in 0..hands_len {
        hands[i].rank = hands_len as u64 - i as u64;
    }
    println!("hands: {:#?}", hands[0..3].to_vec());

    let answer = hands.iter().map(|h| h.bid * h.rank).sum::<u64>();
    println!("answer: {}", answer)
}
