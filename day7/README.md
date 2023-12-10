# day7

to run:
```bash
cargo build --release && time target/release/day7 [1|2]
```

to develop:
```bash
cargo watch -x "run --release -- [1|2]"
```

## Output
### Part 1
```
Hello, adventofcode.com/2023/day/7 from rust!
--- Part One ---
hands: [
    Hand {
        cards: "JJJJJ",
        card_ranks: [
            9,
            9,
            9,
            9,
            9,
        ],
        typ: "Five of a Kind",
        type_rank: 6,
        rank: 1000,
        bid: 90,
    },
    Hand {
        cards: "AAAAJ",
        card_ranks: [
            12,
            12,
            12,
            12,
            9,
        ],
        typ: "Four of a Kind",
        type_rank: 5,
        rank: 999,
        bid: 817,
    },
    Hand {
        cards: "AAAAT",
        card_ranks: [
            12,
            12,
            12,
            12,
            8,
        ],
        typ: "Four of a Kind",
        type_rank: 5,
        rank: 998,
        bid: 19,
    },
]
answer: 248812215
target/release/day7  0.00s user 0.00s system 3% cpu 0.129 total
```

### Part 2
```
Hello, adventofcode.com/2023/day/7 from rust!
--- Part Two ---
hands: [
    Hand {
        cards: "AAAAJ",
        card_ranks: [
            12,
            12,
            12,
            12,
            0,
        ],
        typ: "Five of a Kind",
        type_rank: 6,
        rank: 1000,
        bid: 817,
    },
    Hand {
        cards: "KKKJK",
        card_ranks: [
            11,
            11,
            11,
            0,
            11,
        ],
        typ: "Five of a Kind",
        type_rank: 6,
        rank: 999,
        bid: 486,
    },
    Hand {
        cards: "KKKJJ",
        card_ranks: [
            11,
            11,
            11,
            0,
            0,
        ],
        typ: "Five of a Kind",
        type_rank: 6,
        rank: 998,
        bid: 594,
    },
]
answer: 250057090
target/release/day7 2  0.00s user 0.00s system 5% cpu 0.073 total
```