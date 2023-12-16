#[allow(dead_code)]
static EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

/*Start by tilting the lever so all of the rocks will slide north as far as they will go:
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
*/

// the following 4 functions are unnecessarily optimized
// it's 4 times the same thing but going in different directions
// it would not be easy to generalize this to any direction and keeping
// zero allocation and zero copying. How to do this in Rust?
// But with the added trick of finding the pattern, this optimization is pointless
// just having one function and shuffling the rows around would be enough (see below main).

// move 'O' up north
// go from last row to first row
// go from left col to right col
fn tilt_north(rows: &mut Vec<Vec<char>>, num_rows: usize, num_cols: usize) {
    for r_i in (0..num_rows).rev() {
        for c_i in 0..num_cols {
            let c = rows[r_i][c_i];
            if c == 'O' {
                for r_i2 in (0..r_i).rev() {
                    let c2 = rows[r_i2][c_i];
                    //println!("{}.{}:{} -> {}.{}:{}", r_i, c_i, c, r_i2, c_i, c2);
                    if c2 == '.' {
                        rows[r_i2][c_i] = 'O';
                        rows[r_i][c_i] = '.';
                        break;
                    } else if c2 == 'O' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

// same as tilt_north but rows reversed
fn tilt_south(rows: &mut Vec<Vec<char>>, num_rows: usize, num_cols: usize) {
    for r_i in 0..num_rows {
        for c_i in 0..num_cols {
            let c = rows[r_i][c_i];
            if c == 'O' {
                for r_i2 in r_i + 1..num_rows {
                    let c2 = rows[r_i2][c_i];
                    //println!("{}.{}:{} -> {}.{}:{}", r_i, c_i, c, r_i2, c_i, c2);
                    if c2 == '.' {
                        rows[r_i2][c_i] = 'O';
                        rows[r_i][c_i] = '.';
                        break;
                    } else if c2 == 'O' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

// go from right to left col wise
// go from first row to last row
fn tilt_west(rows: &mut Vec<Vec<char>>, num_rows: usize, num_cols: usize) {
    for c_i in (0..num_cols).rev() {
        for r_i in 0..num_rows {
            let c = rows[r_i][c_i];
            if c == 'O' {
                for c_i2 in (0..c_i).rev() {
                    let c2 = rows[r_i][c_i2];
                    //println!("{}.{}:{} -> {}.{}:{}", r_i, c_i, c, r_i, c_i2, c2);
                    if c2 == '.' {
                        rows[r_i][c_i2] = 'O';
                        rows[r_i][c_i] = '.';
                        break;
                    } else if c2 == 'O' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

// same as tilt_west but cols reversed
fn tilt_east(rows: &mut Vec<Vec<char>>, num_rows: usize, num_cols: usize) {
    for c_i in 0..num_cols {
        for r_i in 0..num_rows {
            let c = rows[r_i][c_i];
            if c == 'O' {
                for c_i2 in c_i + 1..num_cols {
                    let c2 = rows[r_i][c_i2];
                    //println!("{}.{}:{} -> {}.{}:{}", r_i, c_i, c, r_i, c_i2, c2);
                    if c2 == '.' {
                        rows[r_i][c_i2] = 'O';
                        rows[r_i][c_i] = '.';
                        break;
                    } else if c2 == 'O' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn get_load_beam_north(rows: &Vec<Vec<char>>) -> u32 {
    rows.iter()
        .enumerate()
        .map(|(i, r)| (r.iter().filter(|c| **c == 'O').count() * (rows.len() - i)) as u32)
        .sum::<u32>()
}

//check for recurring pattern in measurements
// [0,2,2,7,1,3,2,2,7,1,3,2] => pattern is: [2,2,7,1,3]
// start at 1, eg.: 2
// search for equal num further down the list
// if found, check if all numbers in between are equal to the numbers following
// if a repeating pattern is found, break out of loop
// returns the pattern and the index it starts at
fn search_pattern_in_numbers(numbers: &Vec<u32>) -> Option<(Vec<u32>, usize)> {
    for i in 0..numbers.len() {
        let num = numbers[i];
        // search equal num further down
        for j in i + 2..numbers.len() {
            let next_index_relative = numbers[j..]
                .iter()
                .enumerate()
                .find(|(_, n)| **n == num)
                .map(|(i, _)| i);

            if let Some(next_index_relative) = next_index_relative {
                let next_index = j + next_index_relative;
                let period_length = next_index - i;
                if next_index + period_length > numbers.len() {
                    continue;
                }
                if numbers[i..next_index]
                    .iter()
                    .zip(numbers[next_index..next_index + period_length].iter())
                    .all(|(num1, num2)| num1 == num2)
                {
                    //println!("found pattern: {:?} at i: {}", &numbers[i..next_index], i);
                    return Some((numbers[i..next_index].to_vec(), i));
                }
            }
        }
    }
    None
}

fn main() {
    //let input = EXAMPLE;
    let input = include_str!("../input.txt");
    //println!("input:\n{}", input);

    let part = 2;
    println!("part: {}", part);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        rows.push(line.chars().collect());
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    let mut measurements: Vec<u32> = Vec::new();
    let mut pattern: Vec<u32> = Vec::new();
    let mut pattern_start_index = 0;
    let mut pattern_found_at_iteration = 0;

    // 1 billion 😱
    for i in 0..1000000000 {
        tilt_north(&mut rows, num_rows, num_cols);
        tilt_west(&mut rows, num_rows, num_cols);
        tilt_south(&mut rows, num_rows, num_cols);
        tilt_east(&mut rows, num_rows, num_cols);

        let load_beam_north = get_load_beam_north(&rows);

        measurements.push(load_beam_north);

        let new_pattern = search_pattern_in_numbers(&measurements);
        if let Some(new_pattern) = new_pattern {
            pattern = new_pattern.0;
            pattern_start_index = new_pattern.1;
            pattern_found_at_iteration = i;
            break;
        }
    }

    println!(
        "found pattern: {:?}\nat start_index: {} pattern_found_at_iteration:{}",
        pattern, pattern_start_index, pattern_found_at_iteration
    );
    //println!("measurements: {:?}", measurements);
    let remaining_into_pattern = (1000000000 - pattern_start_index - 1) % pattern.len();
    println!("remaining_into_pattern: {}", remaining_into_pattern);
    println!("answer: {}", pattern[remaining_into_pattern]);
}

//
//
//
//
// simpler version of the 4 tilt functions above:
//
// fn pivot_rows(rows: &mut Vec<Vec<char>>) {
//     let mut new_rows: Vec<Vec<char>> = Vec::new();
//     for c_i in 0..rows.iter().next().unwrap().len() {
//         let mut new_row: Vec<char> = Vec::new();
//         for r_i in 0..rows.len() {
//             new_row.push(rows[r_i][c_i]);
//         }
//         new_rows.push(new_row);
//     }
//     *rows = new_rows;
// }
//
// fn reverse_rows(rows: &mut Vec<Vec<char>>) {
//     *rows = rows.iter().rev().map(|r| r.clone()).collect();
// }

// usage one cycle:

// tilt_rows(&mut rows);

// pivot_rows(&mut rows);
// tilt_rows(&mut rows);
// pivot_rows(&mut rows);

// reverse_rows(&mut rows);
// tilt_rows(&mut rows);
// reverse_rows(&mut rows);

// pivot_rows(&mut rows);
// reverse_rows(&mut rows);
// tilt_rows(&mut rows);
// reverse_rows(&mut rows);
// pivot_rows(&mut rows);
