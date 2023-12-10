#[allow(dead_code)]
static EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug)]
struct AlmanacMapRange {
    destination_range: u32,
    source_range: u32,
    range_length: u32,
}

#[derive(Debug)]
struct AlmanacMap {
    #[allow(dead_code)] // actually used when logging maps
    title: String,
    ranges: Vec<AlmanacMapRange>,
}

fn parse_almanac(almanac: &str) -> (Vec<u32>, Vec<AlmanacMap>) {
    let mut seeds: Vec<u32> = Vec::new();
    let mut maps: Vec<AlmanacMap> = Vec::new();

    // parse the almanac into seeds and maps
    for line in almanac.lines() {
        if line.trim().len() == 0 {
            continue;
        }
        // all the seeds
        if line.starts_with("seeds: ") {
            seeds = line[7..]
                .split(' ')
                .filter(|s| s.len() > 0 && s != &" ")
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<u32>>()
        }
        // start of a map
        else if line.contains(":") {
            maps.push(AlmanacMap {
                title: line[0..line.len() - 5].to_string(),
                ranges: Vec::new(),
            });
        }
        // entry in a map
        else {
            let fields = line
                .split(' ')
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<u32>>();
            let range = AlmanacMapRange {
                destination_range: fields[0],
                source_range: fields[1],
                range_length: fields[2],
            };
            let map = maps.last_mut().unwrap();
            map.ranges.push(range);
        }
    }
    (seeds, maps)
}

// goes from seed through all maps and returns the location (last map's entry)
fn get_destination(seed: u32, maps: &Vec<AlmanacMap>) -> u32 {
    let mut current_value = seed;
    for map in maps.iter() {
        if let Some(map_range) = map.ranges.iter().find(|r| {
            r.source_range <= current_value && current_value < r.source_range + r.range_length
        }) {
            // jump to mapped destination
            current_value = map_range.destination_range + (current_value - map_range.source_range);
        }
        // else: outside of mapped values, current_value stays as is
    }
    current_value
}

fn main() {
    println!("Hello, adventofcode.com/2023/day/5 from rust!");
    let args = std::env::args().collect::<Vec<String>>();

    //let almanac = EXAMPLE;
    let almanac = std::fs::read_to_string("input.txt").unwrap();

    // print first 4 lines
    println!(
        "almanac:\n{}",
        almanac.lines().take(4).collect::<Vec<&str>>().join("\n")
    );

    // first param, default 1
    let part = if args.len() > 1 {
        args[1].parse::<u32>().unwrap()
    } else {
        1
    };

    println!("--- Part {} ---", if part == 1 { "One" } else { "Two" });

    let (seeds, maps) = parse_almanac(&almanac);
    println!("seeds: {:?}", seeds);

    let mut min_destination = u32::MAX;
    if part == 1 {
        for seed in seeds {
            let destination = get_destination(seed, &maps);
            if destination < min_destination {
                min_destination = destination;
            }
        }
    } else if part == 2 {
        // this could be made faster by searching from the destinations backwards
        for i in 0..seeds.len() / 2 {
            let start = seeds[i * 2];
            let length = seeds[i * 2 + 1];
            println!(
                "start: {}, length: {}, min_destination: {}",
                start, length, min_destination
            );
            for j in start..start + length as u32 {
                let destination = get_destination(j as u32, &maps);
                if destination < min_destination {
                    min_destination = destination;
                }
            }
        }
    }

    println!("min_destination: {:?}", min_destination);
}
