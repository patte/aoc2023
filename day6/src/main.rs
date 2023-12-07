/*
https://adventofcode.com/2023/day/6
*/
#[allow(dead_code)]
static EXAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200";

/*
This lovely example can "easily" be solved analytically, so let's do that.
t = time (of race): integer (only natural numbers)
dw = winning distance
v = velocity (speed) (how long you press the button)
d = distance (how far you go)

//distance is the time you pressed the button multiplied by remaining time
dw = v * ( t - v )
   = -v^2 + (t * v)

// solve for v (with wolfram alpha)
v1 = 1/2 * (t - sqrt(t^2 - 4 * dw))
v2 = 1/2 * (t + sqrt(t^2 - 4 * dw))

// v1 is the min time to hold button to win, v2 the max time
// is v2 = t - v1? yes:
v2 =? t - v1
=  t - 1/2 * (t - sqrt(t^2 - 4 * dw))
=  1/2 * (t + sqrt(t^2 - 4 * dw))
=  v2

// first example:
t = 7
dw = 9
x = 4
(v = 2, 3, 4, or 5)

v1 = 1/2 * (7 - sqrt(7^2 - 4 * 9))
   = 1.6972

v2 = 1/2 * (7 + sqrt(7^2 - 4 * 9))
   = 5.3028

// bounds:
v_winning_start > v1
v_winning_end < v2
// it's only winning if we get further than dw
v_winning_start = floor(v1) + 1 // the first possible v to win
v_winning_end = ceil(v2) - 1 // the last possible v to win
// floor and +1 so it also works for v1 = 3.0 (full integers)

// how many possible t (integers) exist to win?
x = possible t values to win

x = ceil(v2) - 1 - (floor(v1) + 1) + 1 // last +1 is because we also count the last v (v2)
    = ceil(v2) - floor(v1) - 1
    = ceil(t - v1) - floor(v1) - 1

🛶 🎉
*/

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    line[7..]
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn main() {
    //let input = EXAMPLE;
    let input = std::fs::read_to_string("input.txt").unwrap();

    let first_line = input.lines().next().unwrap();
    let second_line = input.lines().nth(1).unwrap();

    let times = get_numbers_from_line(first_line);
    let distances = get_numbers_from_line(second_line);

    let mut mul: u32 = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let t = *time as f32;
        let dw = *distance as f32;
        let v1 = 0.5 * (t - (t * t - 4.0 * dw).sqrt());
        let x = ((t - v1).ceil() - v1.floor() - 1.0) as u32;
        println!("t: {}, dw: {}, x: {}, v1: {}", t, dw, x, v1);
        mul *= x;
    }
    println!("answer: {}", mul);
}
