use std::{
    fs::File,
    io::{BufReader, Read},
    iter::Peekable,
    slice::Iter,
};

static EXAMPLE: &str = "seeds: 79 14 55 13

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
struct Transform {
    to: u64,
    from: u64,
    range: u64,
}

struct Mapping {
    transforms: Vec<Transform>,
}

struct Table {
    mappings: Vec<Mapping>,
}

impl Table {
    fn apply(&self, start: u64, n: u64) -> u64 {
        let m = (start..(start + n))
            .map(|mut seed| {
                self.mappings.iter().for_each(|m| {
                    seed = m.apply(seed);
                });
                seed
            })
            .min()
            .unwrap();
        println!("chunk_min: {m}");

        m
    }
}

impl Mapping {
    fn apply(&self, seed: u64) -> u64 {
        self.transforms
            .iter()
            .find(|t| t.contains(seed))
            .map_or(seed, |t| t.apply(seed))
    }
}

impl Transform {
    fn contains(&self, n: u64) -> bool {
        n >= self.from && n < (self.from + self.range)
    }

    fn apply(&self, n: u64) -> u64 {
        self.to + (n - self.from)
    }
}

fn main() {
    let file = File::open("../../inputs/05.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    // run(EXAMPLE)
    run(&input)
}

fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let seeds: Vec<u64> = nsplit(lines[0].split(":").last().unwrap());
    let mut iter = lines[2..].iter().peekable();

    let table = Table {
        mappings: (0..7).map(|_| gather(&mut iter)).collect(),
    };

    let solution: u64 = seeds
        .iter()
        .map(|seed| table.apply(*seed, 1))
        .min()
        .unwrap();
    println!("part 1: {solution:?}");

    let solution = seeds
        .chunks_exact(2)
        .map(|chunk| table.apply(chunk[0], chunk[1]))
        .min()
        .unwrap();
    println!("part 2: {solution:?}");
}

fn gather(iter: &mut Peekable<Iter<'_, &str>>) -> Mapping {
    iter.next().unwrap();

    let mut transforms: Vec<Transform> = Vec::new();
    while !iter
        .peek()
        .map(|l| l.eq_ignore_ascii_case(""))
        .unwrap_or(true)
    {
        let arr = nsplit(iter.next().unwrap());
        let transform = Transform {
            to: arr[0],
            from: arr[1],
            range: arr[2],
        };
        transforms.push(transform);
    }
    iter.next();

    transforms.sort_by(|a, b| a.from.cmp(&b.from));
    Mapping { transforms }
}

fn nsplit(input: &str) -> Vec<u64> {
    input
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| u64::from_str_radix(s, 10).expect(s))
        .collect()
}
