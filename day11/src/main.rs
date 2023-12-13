#[allow(dead_code)]
static EXAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
// This path has length 9 because it takes a minimum of nine steps to get
// from galaxy 5 to galaxy 9 (the eight locations marked # plus the step
// onto galaxy 9 itself). Here are some other example shortest path lengths:
// Between galaxy 5 and 9: 9
// Between galaxy 1 and 7: 15
// Between galaxy 3 and 6: 17
// Between galaxy 8 and 9: 5
// the sum of the shortest path between all 36 pairs of galaxies is 374.

#[derive(Debug, Clone, Copy, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

// implement &Vector + &Vector
impl<'a, 'b> std::ops::Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// implement Vector - Vector
impl std::ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// implement Vector == Vector
impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

// to draw pretty ascii art output (not relevant for solution)
fn set_char_at_position(position: &Vector, input: &str, c: char) -> String {
    let mut lines: Vec<&str> = input.lines().collect();
    let line = lines.get_mut(position.y as usize).unwrap();
    let mut chars: Vec<char> = line.chars().collect();
    chars[position.x as usize] = c;
    let new_line: String = chars.into_iter().collect();
    lines[position.y as usize] = &new_line;
    lines.join("\n")
}

fn double_lines_with_only_symbol(symbol: char, input: &str) -> String {
    let mut new_lines: Vec<String> = Vec::new();
    for line in input.lines() {
        new_lines.push(line.to_string());
        if line.chars().all(|c| c == symbol) {
            new_lines.push(line.to_string());
        }
    }
    new_lines.join("\n")
}

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

#[derive(Debug, Eq, Clone, Copy)]
struct Galaxy {
    id: usize,
    pos: Vector,
}

// implement Galaxy == Galaxy
impl std::cmp::PartialEq for Galaxy {
    fn eq(&self, other: &Galaxy) -> bool {
        self.id == other.id
    }
}

fn main() {
    let test_v1 = Vector { x: 1, y: 2 };
    let test_v2 = Vector { x: 3, y: 4 };
    let test_v3 = Vector { x: 1, y: 2 };
    assert_eq!(&test_v1 + &test_v2, Vector { x: 4, y: 6 });
    assert_eq!(test_v1 == test_v2, false);
    assert_eq!(test_v1 == test_v1, true);
    assert_eq!(test_v1 == test_v3, true);
    assert_eq!(&test_v1 == &test_v2, false);
    assert_eq!(&test_v1 == &test_v1, true);
    assert_eq!(&test_v1 == &test_v3, true);

    //let map = EXAMPLE;
    let map = include_str!("../input.txt");

    println!("input:\n{}", map);

    let part = 1;
    println!("part: {}", part);

    // growing universe
    let mut map = double_lines_with_only_symbol('.', map);
    map = pivot(&map);
    map = double_lines_with_only_symbol('.', &map);
    map = pivot(&map);
    println!("map:\n{}", map);

    // parse map into galaxies
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (row, line) in map.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    id: galaxies.len() + 1,
                    pos: Vector {
                        x: col as i32,
                        y: row as i32,
                    },
                });
            }
        }
    }

    // draw map with galaxy ids
    let mut map_with_galaxy_ids = map.clone();
    for galaxy in &galaxies {
        map_with_galaxy_ids = set_char_at_position(
            &galaxy.pos,
            &map_with_galaxy_ids,
            galaxy.id.to_string().chars().rev().next().unwrap(),
        );
    }
    //println!("map_with_galaxy_ids:\n{}", map_with_galaxy_ids);

    // galaxy pairs
    let mut galaxy_pairs: Vec<(Galaxy, Galaxy)> = Vec::new();
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for galaxy2 in &galaxies[i + 1..] {
            galaxy_pairs.push((*galaxy1, *galaxy2));
        }
    }
    //println!("galaxy_pairs:\n{:?}", galaxy_pairs);
    println!("num galaxy_pairs: {}", galaxy_pairs.len());

    // galaxy pair vectors
    let mut galaxy_pair_vectors: Vec<(Galaxy, Galaxy, Vector)> = Vec::new();
    for (galaxy1, galaxy2) in &galaxy_pairs {
        let v = galaxy2.pos - &galaxy1.pos;
        galaxy_pair_vectors.push((*galaxy1, *galaxy2, v));
    }
    //println!("galaxy_pair_vectors:\n{:?}", galaxy_pair_vectors);

    // find shortest path between all pairs of galaxies,
    // one step can only be made in one direction of either: up, down, left, right
    // v is normalized vector, the loops for x and y need to rasterize the vector onto the map
    let shortest_paths = galaxy_pair_vectors
        .iter()
        .map(|(_, _, v)| v.x.abs() + v.y.abs());

    let sum_of_shortest_paths = shortest_paths.clone().sum::<i32>();
    println!("sum_of_shortest_paths: {}", sum_of_shortest_paths);

    // draw map with galaxy ids and shortest path
    let mut map_with_galaxy_ids_and_shortest_path = map_with_galaxy_ids.clone();
    for (galaxy1, galaxy2, v) in galaxy_pair_vectors.iter() {
        if galaxy1.id == 5 && galaxy2.id == galaxies.len() - 2 {
            let steps_x = v.x.abs();
            let steps_y = v.y.abs();
            for step in 1..steps_x.max(steps_y) {
                let step_v = Vector {
                    x: (v.x as f32 / steps_x as f32 * step.min(steps_x) as f32).round() as i32,
                    y: (v.y as f32 / steps_y as f32 * step.min(steps_y) as f32).round() as i32,
                };
                let step_pos = &galaxy1.pos + &step_v;
                map_with_galaxy_ids_and_shortest_path = set_char_at_position(
                    &step_pos,
                    &map_with_galaxy_ids_and_shortest_path,
                    //std::char::from_digit((galaxy1.id) as u32, 10).unwrap(),
                    'X',
                );
            }
        }
    }
    println!(
        "map_with_galaxy_ids_and_shortest_path:\n{}",
        map_with_galaxy_ids_and_shortest_path
    );
}
