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
    title: String,
    ranges: Vec<AlmanacMapRange>,
}

fn main() {
    println!("Hello, adventofcode.com/2023/day/5 from rust!");

    let almanac = EXAMPLE;
    //let almanac = std::fs::read_to_string("input.txt").unwrap();

    // print first 4 lines
    println!(
        "almanac:\n{}\n---------------------",
        almanac.lines().take(4).collect::<Vec<&str>>().join("\n")
    );

    let mut seeds: Vec<u32> = Vec::new();
    let mut maps: Vec<AlmanacMap> = Vec::new();

    // parse the almanac into seeds and maps
    for line in almanac.lines() {
        if line.trim().len() == 0 {
            continue;
        } else if line.starts_with("seeds: ") {
            seeds = line[7..]
                .split(' ')
                .filter(|s| s.len() > 0 && s != &" ")
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<u32>>()
        } else if line.contains(":") {
            maps.push(AlmanacMap {
                title: line[0..line.len() - 5].to_string(),
                ranges: Vec::new(),
            });
        } else {
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
    println!("seeds: {:?}", seeds);
    //println!("maps: {:#?}", maps);

    let mut destinations: Vec<u32> = Vec::new();
    // go from seed to destination by consulting each map
    for seed in seeds {
        let mut current_value = seed;
        for map in maps.iter() {
            let map_range: Option<&AlmanacMapRange> = map.ranges.iter().find(|r| {
                r.source_range <= current_value && current_value <= r.source_range + r.range_length
            });
            if map_range.is_some() {
                let map_range = map_range.unwrap();
                // jump
                current_value =
                    map_range.destination_range + (current_value - map_range.source_range);
            }
            // else: the current_value stays the same
        }
        destinations.push(current_value);
    }
    let min_destination = destinations.iter().min().unwrap();

    println!("destinations: {:?}", destinations);
    println!("min_destination: {:?}", min_destination);
}
