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

#[derive(Debug)]
struct Pattern {
    pattern_string: String,
    #[allow(dead_code)]
    rows: Vec<(usize, String)>,
    reflection_index: Option<usize>,
    is_rotated: bool,
}

impl Pattern {
    fn new(pattern_string: &str) -> Pattern {
        let rows = pattern_string
            .lines()
            .enumerate()
            .map(|(i, line)| (i, line.to_string()))
            .collect();
        let reflection_index = Pattern::find_horizontal_reflection(&rows);
        Pattern {
            pattern_string: pattern_string.to_string(),
            rows,
            reflection_index,
            is_rotated: false,
        }
    }

    fn new_rotated(pattern_string: &str) -> Pattern {
        let mut pattern = Pattern::new(pattern_string);
        pattern.is_rotated = true;
        pattern
    }

    fn rotate(&self) -> Pattern {
        let rotated = pivot(&self.pattern_string);
        Pattern::new_rotated(&rotated)
    }

    // finds the horizontal reflection in the pattern and
    // returns the index of the row which the reflection is just after
    fn find_horizontal_reflection(rows: &Vec<(usize, String)>) -> Option<usize> {
        let mut identical_line_groups: Vec<(String, Vec<usize>)> = Vec::new();

        for (i, row) in rows.iter().enumerate() {
            let mut found = false;
            for (_, group) in identical_line_groups.iter_mut().enumerate() {
                if group.0 == row.1 {
                    group.1.push(i + 1);
                    found = true;
                    break;
                }
            }
            if !found {
                identical_line_groups.push((row.1.clone(), vec![i + 1]));
            }
        }
        //println!("identical_line_groups: {:?}", identical_line_groups);

        // the first direct reflection (group with consecutive indices)
        // for which all reflection pairs exist is the correct one
        // condition: not multiple reflections
        let mut reflection_between: Option<(usize, usize)> = None;
        for group in identical_line_groups.iter() {
            for (group1_index, group2_index) in group.1.iter().zip(group.1.iter().skip(1)) {
                let diff = group2_index - group1_index;
                if diff == 1 {
                    //println!(
                    //    "possible reflection_between: {:?} {:?}",
                    //    group1_index, group2_index
                    //);
                    let all_pairs_exist = (1..*group1_index)
                        .rev()
                        .zip(group2_index + 1..=rows.len())
                        .all(|(i1, i2)| {
                            identical_line_groups
                                .iter()
                                .any(|group| group.1.contains(&i1) && group.1.contains(&i2))
                        });

                    if all_pairs_exist {
                        reflection_between = Some((*group1_index, *group2_index));
                        break;
                    }
                }
            }
        }

        if let Some(rb) = reflection_between {
            //println!("!!!!reflection_between: {:?}", rb);
            return Some(rb.0);
        } else {
            return None;
        }
    }

    fn print(&self) {
        let mut pattern_string = self.pattern_string.clone();
        if let Some(reflection_index) = self.reflection_index {
            pattern_string = pattern_string
                .split("\n")
                .enumerate()
                .map(|(i, line)| {
                    if i == reflection_index - 1 {
                        let symbol = if !self.is_rotated { "v" } else { ">" };
                        symbol.to_owned() + line + symbol
                    } else if i == reflection_index {
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
        }
        println!("{}", pattern_string);
    }
}

impl std::cmp::PartialEq for Pattern {
    fn eq(&self, other: &Pattern) -> bool {
        self.pattern_string == other.pattern_string
    }
}

fn main() {
    assert_eq!(
        Pattern::new("#..#\n....\n....\n#..#") == Pattern::new("#..#\n....\n....\n#..#"),
        true
    );
    assert_eq!(
        Pattern::new("#..#\n....\n....\n#..#") == Pattern::new("....\n.##.\n.##.\n...."),
        false
    );

    //let patterns_input = EXAMPLE;
    let patterns_input = include_str!("../input.txt");
    //println!("input:\n{}", patterns_input);

    let part = 1;
    println!("part: {}", part);

    // split map on double linebreaks
    let pattern_strings: Vec<&str> = patterns_input.split("\n\n").collect();
    let patterns: Vec<Pattern> = pattern_strings
        .iter()
        .map(|pattern_string| Pattern::new(pattern_string))
        .collect();

    let mut sum_of_reflection_indices = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        println!("pattern {}:", i);
        if let Some(reflection_index) = pattern.reflection_index {
            sum_of_reflection_indices += reflection_index * 100;
            pattern.print();
        } else {
            let pattern_rotated = pattern.rotate();
            if let Some(reflection_index) = pattern_rotated.reflection_index {
                sum_of_reflection_indices += reflection_index;
                pattern_rotated.print();
            } else {
                println!("{:?}", pattern);
                panic!("no reflection found");
            }
        }
    }

    println!("answer: {}", sum_of_reflection_indices);
}
