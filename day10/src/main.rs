use std::slice::Iter;

#[allow(dead_code)]
static EXAMPLE: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

#[allow(dead_code)]
static EXAMPLE2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

#[allow(dead_code)]
static EXAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....";

#[allow(dead_code)]
static EXAMPLE3_1: &str = "\
............
.S--------7.
.|F----7..|.
.||....|..|.
.||..F-J..|.
.|L-7|....|.
.|..||....|.
.L--JL----J.
...........";

#[allow(dead_code)]
static EXAMPLE4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
// answer part2: 8

#[allow(dead_code)]
static EXAMPLE5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
// answer part2: 10

#[allow(dead_code)]
static EXAMPLE6: &str = "\
............
.F7-F-----7.
.|L-J.....|.
.|........|.
.|........|.
.|........|.
.|........|.
.L--------S.
...........";

#[derive(Debug, Clone, Copy, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

// implement &Point + &Point
impl<'a, 'b> std::ops::Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
// implement Point == Point
impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}
// Hash for HashSet by multiplying x and y by primes
impl std::hash::Hash for Vector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Skew {
    Left,
    Right,
}

use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Ordinalize)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ];
        DIRECTIONS.iter()
    }
    pub fn to_vector(&self) -> Vector {
        match self {
            Direction::East => Vector { x: 1, y: 0 },
            Direction::South => Vector { x: 0, y: 1 },
            Direction::West => Vector { x: -1, y: 0 },
            Direction::North => Vector { x: 0, y: -1 },
        }
    }
    pub fn get_inverse(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
        }
    }
    pub fn get_skew(&self, &other: &Direction) -> Option<Skew> {
        // if going straight, no skew
        if other == self.get_skewed_by(&Skew::Left) {
            return Some(Skew::Left);
        }
        if other == self.get_skewed_by(&Skew::Right) {
            return Some(Skew::Right);
        }
        None
    }

    pub fn get_skewed_by(&self, skew: &Skew) -> Direction {
        let self_ord = self.ordinal();
        let new_ord = match skew {
            Skew::Right => (self_ord + 1) % 4,
            Skew::Left => (self_ord + 3) % 4,
        };
        Direction::from_ordinal(new_ord).unwrap_or_else(|| {
            panic!(
                "Could not get skewed direction for {:?} and skew {:?}",
                self, skew
            )
        })
    }
}

static PIPES: [char; 6] = ['|', '-', 'L', 'J', 'F', '7'];

#[derive(Debug)]
struct Pipe {
    #[allow(dead_code)]
    symbol: char,
    directions: Vec<Direction>,
}

impl Pipe {
    pub fn is_pipe(symbol: &char) -> bool {
        PIPES.contains(symbol)
    }

    pub fn directions(symbol: &char) -> Vec<Direction> {
        match symbol {
            '|' => vec![Direction::North, Direction::South],
            '-' => vec![Direction::East, Direction::West],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::North, Direction::West],
            '7' => vec![Direction::South, Direction::West],
            'F' => vec![Direction::South, Direction::East],
            _ => panic!("Unknown pipe: {}", symbol),
        }
    }

    pub fn new(symbol: char) -> Pipe {
        Pipe {
            symbol,
            directions: Pipe::directions(&symbol),
        }
    }

    pub fn try_new(symbol: char) -> Option<Pipe> {
        if Pipe::is_pipe(&symbol) {
            Some(Pipe::new(symbol))
        } else {
            None
        }
    }

    // checks if the pipe connects from the given direction
    // returns the direction where the pipe connects to
    // or None
    pub fn connects_from_to(&self, from_direction: &Direction) -> Option<Direction> {
        let inversed_from_direction = from_direction.get_inverse();
        if self.directions.contains(&inversed_from_direction) {
            Some(
                self.directions
                    .iter()
                    .find(|&d| d != &inversed_from_direction)
                    .unwrap()
                    .clone(),
            )
        } else {
            None
        }
    }
}

// simple 2d array transform
fn get_char_at_position(position: &Vector, input: &str) -> Option<char> {
    let lines: Vec<&str> = input.lines().collect();
    if position.y < 0 || position.y >= lines.len() as i32 {
        return None;
    }
    let line = lines.get(position.y as usize).unwrap();
    let chars: Vec<char> = line.chars().collect();
    chars.get(position.x as usize).copied()
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

fn get_first_position_of_symbol(symbol: char, input: &str) -> Option<Vector> {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == symbol {
                return Some(Vector {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }
    None
}

// go from start position and direction to next pipe and follow it, come to next pipe, follow it, ...
// if no next step is possible, or we land on 'S' again, we are done
// return number of steps and the filtered input
fn walk_pipe(
    current_pos: &Vector,
    current_direction: &Direction,
    input: &str,
    count_enclosed_skew: Option<Skew>,
) -> (u32, String, Skew, u32) {
    let mut current_pos = current_pos.clone();
    let mut current_direction = current_direction.clone();
    let mut steps = 0;

    let one_line_length = input.lines().next().unwrap().len();
    let mut map_walked_pipe =
        format!("{}\n", ".".repeat(one_line_length)).repeat(input.lines().count());
    map_walked_pipe = set_char_at_position(&current_pos, &map_walked_pipe, 'S');

    let mut turns: Vec<Skew> = Vec::new();

    let mut enclosed_tiles: std::collections::HashSet<Vector> = std::collections::HashSet::new();

    loop {
        let next_pos = &current_pos + &current_direction.to_vector();
        if let Some(c) = get_char_at_position(&next_pos, input) {
            if c == 'S' && steps > 0 {
                break;
            }
            if let Some(pipe) = Pipe::try_new(c) {
                if let Some(next_direction) = pipe.connects_from_to(&current_direction) {
                    let skew = current_direction.get_skew(&next_direction);
                    if let Some(skew) = skew {
                        turns.push(skew);
                    }

                    // part2
                    // enclosing
                    if let Some(count_enclosed_skew) = count_enclosed_skew {
                        let search_directions = vec![
                            next_direction.get_skewed_by(&count_enclosed_skew),
                            current_direction.get_skewed_by(&count_enclosed_skew),
                        ];

                        for search_direction in search_directions {
                            // from next_pos go in search_direction
                            // if . is found, mark as enclosed
                            let search_pos = &next_pos + &search_direction.to_vector();
                            if let Some(c2) = get_char_at_position(&search_pos, input) {
                                if c2 == '.' {
                                    enclosed_tiles.insert(search_pos.clone());
                                    map_walked_pipe = set_char_at_position(
                                        &search_pos,
                                        &map_walked_pipe,
                                        if c2 == '.' { 'I' } else { c2 },
                                    );
                                }
                            }
                        }
                    }
                    map_walked_pipe = set_char_at_position(&next_pos, &map_walked_pipe, c);
                    //println!("{}", map_walked_pipe);

                    // go to next pipe
                    steps += 1;
                    current_pos = next_pos;
                    current_direction = next_direction;

                    //std::thread::sleep(std::time::Duration::from_millis(1000));
                } else {
                    //println!("no connecting pipe found!");
                    break;
                }
            } else {
                //println!("no pipe found!");
                break;
            }
        }
    }

    // determine skew
    let num_left_turns = turns.iter().filter(|&s| s == &Skew::Left).count() as u32;
    let num_right_turns = turns.len() as u32 - num_left_turns;
    let skew = if num_left_turns > num_right_turns {
        Skew::Left
    } else {
        Skew::Right
    };

    // for each enclosed tile
    // walk in all directions as long as '.' is found
    // count as enclosed tiles
    if enclosed_tiles.len() > 0 {
        for tile_vec in &enclosed_tiles.clone() {
            for direction in Direction::iter() {
                let mut search_pos = tile_vec.clone();
                loop {
                    search_pos = &search_pos + &direction.to_vector();
                    if let Some(c) = get_char_at_position(&search_pos, input) {
                        if c == '.' {
                            enclosed_tiles.insert(search_pos.clone());
                            map_walked_pipe = set_char_at_position(
                                &search_pos,
                                &map_walked_pipe,
                                if c == '.' { 'i' } else { c },
                            );
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    (steps, map_walked_pipe, skew, enclosed_tiles.len() as u32)
}

fn find_entry_and_walk_pipe(input: &str) -> (u32, String, Skew, Vector, Direction) {
    let start_pos: Vector = get_first_position_of_symbol('S', input)
        .unwrap_or_else(|| panic!("No start position found"));

    // try to walk pipe in all directions
    for direction in Direction::iter() {
        let (num_steps, map_walked_pipe, skew, _) = walk_pipe(&start_pos, &direction, input, None);
        if num_steps > 0 {
            return (
                num_steps,
                map_walked_pipe,
                skew,
                start_pos,
                direction.clone(),
            );
        }
    }
    (0, String::new(), Skew::Left, start_pos, Direction::East)
}

#[allow(dead_code)]
fn get_svg_for_map(map: &str) -> String {
    // part2
    let bits = map
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if Pipe::is_pipe(&c) || c == 'i' { 1 } else { 0 })
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();
    let svg_path = contour_tracing::array::bits_to_paths(bits, true);
    "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 160 160\" width=\"100%\" height=\"100%\"><path d=\"".to_string()
        + &svg_path
        + "\" fill=\"none\" stroke=\"black\" stroke-width=\"0.3\" /></svg>"
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

    //let map = EXAMPLE2;
    //let map = EXAMPLE2;
    //let map = EXAMPLE3;
    //let map = EXAMPLE4;
    //let map = EXAMPLE5;
    //let map = EXAMPLE6;
    let map = include_str!("../input.txt");

    // print first 4 lines
    println!("input:");
    map.lines().enumerate().take(5).for_each(|(i, line)| {
        if i < 4 {
            println!("{}", line);
        } else {
            println!("...{} lines skipped", map.lines().count() - 5)
        }
    });

    let (num_steps, map_walked_pipe, skew, start_pos, start_direction) =
        find_entry_and_walk_pipe(map);

    // write map_walked_pipe to output.txt
    std::fs::write("output.txt", &map_walked_pipe).unwrap();
    // write svg to output.svg
    std::fs::write("output.svg", &get_svg_for_map(&map_walked_pipe)).unwrap();

    let (_, map_walked_enclosed, _, num_enclosed_tiles) =
        walk_pipe(&start_pos, &start_direction, &map_walked_pipe, Some(skew));

    std::fs::write("output-enclosed.txt", &map_walked_enclosed).unwrap();
    std::fs::write(
        "output-enclosed.svg",
        &get_svg_for_map(&map_walked_enclosed),
    )
    .unwrap();

    println!("walked pipe:\n{}", map_walked_pipe);
    println!("walked enclosed:\n{}", map_walked_enclosed);
    println!("skew: {:?}", skew);
    println!("num_steps: {}", num_steps);
    println!("answer part1: {}", (num_steps as f32 / 2.).ceil() as u32);
    println!("answer part2: {}", num_enclosed_tiles);
}
