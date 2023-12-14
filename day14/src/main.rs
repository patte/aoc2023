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

fn main() {
    //let input = EXAMPLE;
    let input = include_str!("../input.txt");
    //println!("input:\n{}", input);

    let part = 1;
    println!("part: {}", part);

    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        rows.push(line.chars().collect());
    }

    // tilt
    for r_i in (0..rows.len()).rev() {
        let row: String = rows[r_i].iter().collect();
        for c_i in 0..row.len() {
            let c = row.chars().nth(c_i).unwrap();
            if c == 'O' {
                for r_i2 in (0..r_i).rev() {
                    let c2 = rows[r_i2].iter().nth(c_i).unwrap();
                    //println!("{}.{}:{} -> {}.{}:{}", r_i, c_i, c, r_i2, c_i, c2);
                    if c2 == &'.' {
                        rows[r_i2][c_i] = 'O';
                        rows[r_i][c_i] = '.';
                        break;
                    } else if c2 == &'O' {
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    // print
    println!("input:\n{}", input);
    println!(
        "tilted:\n{}",
        rows.iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );

    // calc load
    let load_beam_north = rows
        .iter()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|c| **c == 'O').count() * (rows.len() - i))
        .sum::<usize>();
    println!("load_beam_north: {}", load_beam_north);
}
