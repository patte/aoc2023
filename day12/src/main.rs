#[allow(dead_code)]
static EXAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

/*
???.### 1,1,3 - 1 arrangement
.??..??...?##. 1,1,3 - 4 arrangements
?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
????.#...#... 4,1,1 - 1 arrangement
????.######..#####. 1,6,5 - 4 arrangements
?###???????? 3,2,1 - 10 arrangements

Adding all of the possible arrangement counts together produces a total of 21 arrangements.
*/

fn main() {
    //let input = EXAMPLE;
    let input = include_str!("../input.txt");

    let mut counter = 0;

    for line in input.lines() {
        //println!("input:\n{}", line);
        let (pattern, groups) = line.split_at(line.find(' ').unwrap());

        let patterns_to_insert: Vec<(usize, &str)> = pattern
            .split('?')
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .collect::<Vec<(usize, &str)>>();
        //println!("patterns_to_insert: {:?}", patterns_to_insert);

        let num_questionmarks = pattern.matches('?').count();
        //println!("num_questionmarks: {}", num_questionmarks);

        for i in 0..=(2_u32.pow(num_questionmarks as u32) - 1) {
            let i_bin = format!("{:b}", i);
            let i_bin = format!("{:0>width$}", i_bin, width = num_questionmarks);
            let mut num_inserted = 0;
            let mut i_bin_inserted = i_bin.clone();
            for pattern_to_insert in patterns_to_insert.iter() {
                let splitted = i_bin_inserted.split_at(pattern_to_insert.0 + num_inserted);
                i_bin_inserted = format!("{}{}{}", splitted.0, pattern_to_insert.1, splitted.1)
                    .replace("#", "1")
                    .replace(".", "0");
                num_inserted += pattern_to_insert.1.len();
            }

            //println!("i: {} i_bin: {:?}  inserted: {}", i, i_bin, i_bin_inserted);

            let measured_groups = i_bin_inserted
                .split('0')
                .filter(|s| !s.is_empty())
                .map(|s| s.len().to_string())
                .collect::<Vec<String>>()
                .join(",");

            //println!("measured_groups: {:?}", measured_groups);

            if measured_groups == groups.trim() {
                //println!("✅");
                counter += 1;
            } else {
                //println!("❌");
            }
        }

        //println!();
    }

    println!("counter: {}", counter);
}
