# day9

I asked chatGPT4 about how to think about this problem, [but it did not come up](chatGPT4.txt) with a different approach than the one outlined in the problem statement (build up difference pyramids).

to run:
```bash
cargo build --release && time target/release/day9 [1|2]
```

to develop:
```bash
cargo watch -x "run --release [1|2]"
```

## Output
### Part 1
```
✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫
Hello, adventofcode.com/2023/day/9 from rust!
--- Part One ---
histories:
  [3, 8, 13, 18, 23, 28, 33, 38, 43, 48, 53, 58, 63, 68, 73, 78, 83, 88, 93, 98, 103]
  [-2, 11, 29, 53, 88, 153, 309, 726, 1835, 4661, 11532, 27538, 63421, 141097, 303903, 635229, 1292012, 2564703, 4984683, 9517003, 17906238]
  [18, 37, 70, 125, 217, 371, 619, 998, 1568, 2495, 4304, 8537, 19327, 46991, 116021, 282572, 672229, 1560456, 3541417, 7876620, 17202960]
  [5, 3, 9, 41, 127, 305, 637, 1261, 2529, 5316, 11643, 25862, 56858, 122127, 255336, 520269, 1036193, 2023005, 3879505, 7315347, 13567333]
  ...
⏅ levels: 2
⏅ levels: 16
⏅ levels: 18
...
⏅ levels: 11
⏅ levels: 5
⏅ levels: 15
answer: 1974232246
0.00s user 0.00s system 9% cpu 0.071 total
```

### Part 2
```
✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫
Hello, adventofcode.com/2023/day/9 from rust!
--- Part Two ---
histories:
  [3, 8, 13, 18, 23, 28, 33, 38, 43, 48, 53, 58, 63, 68, 73, 78, 83, 88, 93, 98, 103]
  [-2, 11, 29, 53, 88, 153, 309, 726, 1835, 4661, 11532, 27538, 63421, 141097, 303903, 635229, 1292012, 2564703, 4984683, 9517003, 17906238]
  [18, 37, 70, 125, 217, 371, 619, 998, 1568, 2495, 4304, 8537, 19327, 46991, 116021, 282572, 672229, 1560456, 3541417, 7876620, 17202960]
  [5, 3, 9, 41, 127, 305, 637, 1261, 2529, 5316, 11643, 25862, 56858, 122127, 255336, 520269, 1036193, 2023005, 3879505, 7315347, 13567333]
  ...
⏅ levels: 2
⏅ levels: 16
⏅ levels: 18
...
⏅ levels: 11
⏅ levels: 5
⏅ levels: 15
answer: 928
0.00s user 0.00s system 9% cpu 0.070 total
```