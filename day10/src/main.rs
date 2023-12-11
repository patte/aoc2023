use std::slice::Iter;

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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
}

static PIPES: [char; 7] = ['|', '-', 'L', 'J', 'F', '7', '.'];

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
            '.' => vec![],
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
fn walk_pipe(current_pos: &Vector, current_direction: &Direction, input: &str) -> (u32, String) {
    let mut current_pos = current_pos.clone();
    let mut current_direction = current_direction.clone();
    let mut steps = 0;

    let one_line_length = input.lines().next().unwrap().len();
    let mut ascii_output =
        format!("{}\n", ".".repeat(one_line_length)).repeat(input.lines().count());
    ascii_output = set_char_at_position(&current_pos, &ascii_output, 'S');

    loop {
        let next_pos = &current_pos + &current_direction.to_vector();
        if let Some(c) = get_char_at_position(&next_pos, input) {
            //println!("{:?} is: {:?} char: {}", current_direction, next_pos, c);
            if c == 'S' && steps > 0 {
                break;
            }
            if let Some(pipe) = Pipe::try_new(c) {
                if let Some(next_direction) = pipe.connects_from_to(&current_direction) {
                    //println!(
                    //    "connecting pipe found!: {:?} next_direction: {:?}",
                    //    pipe, next_direction
                    //);
                    steps += 1;
                    current_pos = next_pos;
                    current_direction = next_direction;
                    ascii_output = set_char_at_position(&next_pos, &ascii_output, c);
                }
            }
        }
    }
    (steps, ascii_output)
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

    //let input = EXAMPLE2;
    //let input = EXAMPLE2;
    let input = include_str!("../input.txt");

    // print first 4 lines
    println!("input:");
    input.lines().enumerate().take(5).for_each(|(i, line)| {
        if i < 5 {
            println!("{}", line);
        } else {
            println!("{} lines skipped", input.lines().count() - 5)
        }
    });

    let width = EXAMPLE.lines().next().unwrap().len();
    let height = EXAMPLE.lines().count();
    println!("width: {}, height: {}", width, height);

    let start_pos: Vector = get_first_position_of_symbol('S', input)
        .unwrap_or_else(|| panic!("No start position found"));

    println!("start_pos: {:?}", start_pos);

    for direction in Direction::iter() {
        let (num_steps, ascii_output) = walk_pipe(&start_pos, &direction, input);
        println!("walked pipe:\n{}", ascii_output);
        if num_steps > 0 {
            println!("num_steps: {}", num_steps);
            println!("answer: {}", (num_steps as f32 / 2.).ceil() as u32);
            break;
        }
    }
}
