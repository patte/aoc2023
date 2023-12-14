# day14

to run:

```bash
cargo build --release && time target/release/day14
```

to develop:

```bash
RUST_BACKTRACE=1 cargo watch -x "run" --ignore "output*"
```

### Example
```
part: 1
input:
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
tilted:
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
load_beam_north: 136
```

### Part 1
```
load_beam_north: 108614
...
