# day8

to run:
```bash
cargo build --release && time target/release/day8 [1|2]
```

to develop:
```bash
cargo watch -x "run --release [1|2]"
```

## Output
### Part 1
```
✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫
Hello, adventofcode.com/2023/day/8 from rust!
--- Part One ---
directions: LRLRLRLLRRLRRLRRRLRRLRLLRRRLRRRLRRLLLLRRRLRLLRRLRRLRRLLLRRRLRRRLRRLRLRRLRLRLRLLRRRLRRRLLRRRLRRRLRRRLRLLLRRLRLRRRLRLRRRLLRRRLRLLRLRRRLRLRRRLRRLLRLRLRRLRLRLRRLRLRLRRRLRRLRLLRRLRRRLRRRLRRLRRRLRRLRLRRRLLRRRLLRRLRLRRRLRRRLLRRRLRLRRLRLRLRRLRLLRRLRLRLRRLRRRLRRRLRLRRLRRLLLRRRLLRLRRRLLRRRR
AAA-[L]->FHJ 1
FHJ-[R]->TGV 2
TGV-[L]->PVS 3
PVS-[R]->TJC 4
...    ->ZZZ 22199
steps: 22199
target/release/day8 1  0.00s user 0.00s system 4% cpu 0.075 total
```

### Part 2
```
✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫 ✨ ⭐️ ✨ 💫 ✨ ✨ 💫
Hello, adventofcode.com/2023/day/8 from rust!
--- Part Two ---
directions: LRLRLRLLRRLRRLRRRLRRLRLLRRRLRRRLRRLLLLRRRLRLLRRLRRLRRLLLRRRLRRRLRRLRLRRLRLRLRLLRRRLRRRLLRRRLRRRLRRRLRLLLRRLRLRRRLRLRRRLLRRRLRLLRLRRRLRLRRRLRRLLRLRLRRLRLRLRRLRLRLRRRLRRLRLLRRLRRRLRRRLRRLRRRLRRLRLRRRLLRRRLLRRLRLRRRLRRRLLRRRLRLRRLRLRLRRLRLLRRLRLRLRRLRRRLRRRLRLRRLRRLLLRRRLLRLRRRLLRRRR
PXA-[L]->KBG 1
KBG-[R]->HFS 2
HFS-[L]->HML 3
HML-[R]->SVV 4
...    ->LHZ 14893
VXA-[L]->BRF 1
BRF-[R]->LNH 2
LNH-[L]->SHB 3
SHB-[R]->SDJ 4
...    ->TNZ 16579
DVA-[L]->XXG 1
XXG-[R]->MVX 2
MVX-[L]->RBK 3
RBK-[R]->XMX 4
...    ->XDZ 13207
NMA-[L]->KGM 1
KGM-[R]->DHM 2
DHM-[L]->RPP 3
RPP-[R]->VVM 4
...    ->JFZ 17141
AAA-[L]->FHJ 1
FHJ-[R]->TGV 2
TGV-[L]->PVS 3
PVS-[R]->TJC 4
...    ->ZZZ 22199
JHA-[L]->BXQ 1
BXQ-[R]->MVV 2
MVV-[L]->FPF 3
FPF-[R]->QNX 4
...    ->FRZ 18827
ghosts: [Ghost { current_node: "LHZ", step_count: 14893, loop_step_count: 0 }, Ghost { current_node: "TNZ", step_count: 16579, loop_step_count: 0 }, Ghost { current_node: "XDZ", step_count: 13207, loop_step_count: 0 }, Ghost { current_node: "JFZ", step_count: 17141, loop_step_count: 0 }, Ghost { current_node: "ZZZ", step_count: 22199, loop_step_count: 0 }, Ghost { current_node: "FRZ", step_count: 18827, loop_step_count: 0 }]
max_step_count: 22199
LHZ-[L]->KQF 14894
KQF-[R]->HVS 14895
HVS-[L]->HML 14896
HML-[R]->SVV 14897
...    ->LHZ 29786
         loop!!!
TNZ-[L]->XKN 16580
XKN-[R]->VFV 16581
VFV-[L]->SHB 16582
SHB-[R]->SDJ 16583
...    ->TNZ 33158
         loop!!!
XDZ-[L]->PRR 13208
PRR-[R]->FHN 13209
FHN-[L]->RBK 13210
RBK-[R]->XMX 13211
...    ->XDZ 26414
         loop!!!
JFZ-[L]->SXH 17142
SXH-[R]->DHM 17143
DHM-[L]->RPP 17144
RPP-[R]->VVM 17145
...    ->JFZ 34282
         loop!!!
skip: Ghost { current_node: "ZZZ", step_count: 22199, loop_step_count: 0 }
FRZ-[L]->RTP 18828
RTP-[R]->MVV 18829
MVV-[L]->FPF 18830
FPF-[R]->QNX 18831
...    ->FRZ 37654
         loop!!!
ghosts: [Ghost { current_node: "LHZ", step_count: 14893, loop_step_count: 14893 }, Ghost { current_node: "TNZ", step_count: 16579, loop_step_count: 16579 }, Ghost { current_node: "XDZ", step_count: 13207, loop_step_count: 13207 }, Ghost { current_node: "JFZ", step_count: 17141, loop_step_count: 17141 }, Ghost { current_node: "ZZZ", step_count: 22199, loop_step_count: 0 }, Ghost { current_node: "FRZ", step_count: 18827, loop_step_count: 18827 }]
max_step_count: 22199
steps: 13334102464297
target/release/day8 2  0.01s user 0.00s system 12% cpu 0.078 total
```