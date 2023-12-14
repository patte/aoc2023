#[allow(dead_code)]
static EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
// 123456789
//     ><
// #.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
//     ><
// 123456789
//

// 1 #...##..# 1
// 2 #....#..# 2
// 3 ..##..### 3
// 4v#####.##.v4
// 5^#####.##.^5
// 6 ..##..### 6
// 7 #....#..# 7

fn get_column(column: usize, input: &str) -> String {
    let mut new_col: Vec<char> = Vec::new();
    for line in input.lines() {
        new_col.push(line.chars().nth(column).unwrap());
    }
    new_col.into_iter().collect()
}

fn pivot(input: &str) -> String {
    let mut new_lines: Vec<String> = Vec::new();
    let w = input.lines().next().unwrap().len();
    for row in 0..w {
        let col = get_column(row, input);
        new_lines.push(col);
    }
    new_lines.join("\n")
}

fn levenstein_distance(a: &str, b: &str) -> usize {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        matrix[i][0] = i;
    }
    for j in 0..=b.len() {
        matrix[0][j] = j;
    }
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                matrix[i][j + 1] + 1,
                std::cmp::min(matrix[i + 1][j] + 1, matrix[i][j] + cost),
            );
        }
    }
    matrix[a.len()][b.len()]
}

#[derive(Debug)]
struct Pattern {
    pattern_string: String,
    #[allow(dead_code)]
    rows: Vec<(usize, String)>,
    reflection_index: Option<usize>,
    is_rotated: bool,
    part: usize,
}

impl Pattern {
    fn new(pattern_string: &str, part: usize) -> Pattern {
        let rows = pattern_string
            .lines()
            .enumerate()
            .map(|(i, line)| (i, line.to_string()))
            .collect();
        let reflection_index = Pattern::find_horizontal_reflection(&rows, part);
        Pattern {
            pattern_string: pattern_string.to_string(),
            rows,
            reflection_index,
            is_rotated: false,
            part,
        }
    }

    fn new_rotated(pattern_string: &str, part: usize) -> Pattern {
        let mut pattern = Pattern::new(pattern_string, part);
        pattern.is_rotated = true;
        pattern
    }

    fn rotate(&self) -> Pattern {
        let rotated = pivot(&self.pattern_string);
        Pattern::new_rotated(&rotated, self.part)
    }

    // finds the horizontal reflection in the pattern and
    // returns the index of the row which the reflection is just after
    // eg. reflection between row 4 and 5 => returns 4
    // this should be refactored as a method on Pattern
    fn find_horizontal_reflection(rows: &Vec<(usize, String)>, part: usize) -> Option<usize> {
        // find consecutive lines, which are identical then check if
        // all pairs of reflections exist
        // take the first fully working reflection
        // all calculations and the result are 0 based (aligned with the rows vector)
        // part 2: max 1 line must be 1 char off to form a reflection
        // part 2: the reflection line must be different from part1
        //for (i, row) in rows.iter().enumerate() {
        //    println!("{} {}", i, row.1);
        //}
        let mut reflection_between: Option<(usize, usize)> = None;
        for (row1, row2) in rows.iter().zip(rows.iter().skip(1)) {
            let mut remaining_fixes: i16 = 1; // only one fix for all reflections

            let pattern_dist = levenstein_distance(&row1.1, &row2.1);
            if part == 1 {
                if pattern_dist != 0 {
                    continue;
                }
            } else {
                if pattern_dist > 1 {
                    continue;
                }
                remaining_fixes -= pattern_dist as i16;
            }
            //println!(
            //    "possible reflection between {} and {}  dist: {}",
            //    row1.0, row2.0, pattern_dist
            //);
            //walk in both directions simultaneously
            let all_pairs_exist =
                (0..row1.0)
                    .rev()
                    .zip((row2.0 + 1)..rows.len())
                    .all(|(i1, i2)| {
                        // println!(
                        //     "i1: {}, i2: {}  dist: {}",
                        //     i1,
                        //     i2,
                        //     levenstein_distance(&rows[i1].1, &rows[i2].1)
                        // );
                        if part == 1 {
                            rows[i1].1 == rows[i2].1
                        } else {
                            let dist = levenstein_distance(&rows[i1].1, &rows[i2].1);
                            if dist > 1 {
                                false
                            } else if dist == 1 {
                                remaining_fixes -= 1;
                                remaining_fixes >= 0
                            } else {
                                true
                            }
                        }
                    });

            // part 2: require at least 1 smudge fix
            if all_pairs_exist && (part == 1 || remaining_fixes == 0) {
                reflection_between = Some((row1.0, row2.0));
                break;
            }
        }

        if let Some(rb) = reflection_between {
            // println!("!!!!reflection_between: {:?}", rb);
            return Some(rb.0);
        } else {
            // println!("no reflection found");
            return None;
        }
    }

    // print numbers 1 based
    fn print(&self) {
        let mut pattern_string = self.pattern_string.clone();
        if let Some(reflection_index) = self.reflection_index {
            pattern_string = pattern_string
                .split("\n")
                .enumerate()
                .map(|(i, line)| {
                    if i == reflection_index {
                        let symbol = if !self.is_rotated { "v" } else { ">" };
                        symbol.to_owned() + line + symbol
                    } else if i == reflection_index + 1 {
                        let symbol = if !self.is_rotated { "^" } else { "<" };
                        symbol.to_owned() + line + symbol
                    } else {
                        " ".to_string() + line + " "
                    }
                })
                .enumerate()
                .map(|(i, line)| format!("{:2}{}", i + 1, line))
                .collect::<Vec<String>>()
                .join("\n");
        }
        if self.is_rotated {
            pattern_string = pivot(&pattern_string);
            // remove first newline
            pattern_string = pattern_string
                .split("\n")
                .skip(1)
                .collect::<Vec<&str>>()
                .join("\n");
        }
        println!("{}", pattern_string);
    }
}

impl std::cmp::PartialEq for Pattern {
    fn eq(&self, other: &Pattern) -> bool {
        self.pattern_string == other.pattern_string
    }
}
//assert_eq!(
//    Pattern::new("#..#\n....\n....\n#..#", 1) == Pattern::new("#..#\n....\n....\n#..#", 1),
//    true
//);
//assert_eq!(
//    Pattern::new("#..#\n....\n....\n#..#", 1) == Pattern::new("....\n.##.\n.##.\n....", 1),
//    false
//);

fn main() {
    //let patterns_input = EXAMPLE;
    let patterns_input = include_str!("../input.txt");
    //println!("input:\n{}", patterns_input);

    let part = 2;
    println!("part: {}", part);

    // split map on double linebreaks
    let pattern_strings: Vec<&str> = patterns_input.split("\n\n").collect();
    let patterns: Vec<Pattern> = pattern_strings
        .iter()
        .map(|pattern_string| Pattern::new(pattern_string, part))
        .collect();

    let mut sum_of_reflection_indices = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        println!("pattern {}:", i);
        if let Some(reflection_index) = pattern.reflection_index {
            // +1 because solution is 1 based
            sum_of_reflection_indices += (reflection_index + 1) * 100;
            pattern.print();
        } else {
            let pattern_rotated = pattern.rotate();
            if let Some(reflection_index) = pattern_rotated.reflection_index {
                // +1 because solution is 1 based
                sum_of_reflection_indices += reflection_index + 1;
                pattern_rotated.print();
            } else {
                println!("{:?}", pattern);
                panic!("no reflection found");
            }
        }
    }

    println!("answer: {}", sum_of_reflection_indices);
}
