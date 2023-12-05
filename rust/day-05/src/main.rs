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

struct Table {
    mappings: Vec<Mapping>,
}

struct Mapping {
    transforms: Vec<Transform>,
}

#[derive(Debug)]
struct Transform {
    to: u64,
    from: u64,
    len: u64,
}

impl Table {
    fn apply(&self, seed: u64, n: u64) -> u64 {
        let intermediate = self
            .mappings
            .iter()
            .fold(vec![(seed, seed + n)], |acc, mapping| {
                acc.iter()
                    .flat_map(|(from, to)| mapping.apply(*from, *to))
                    .collect()
            });

        let m = intermediate.iter().map(|f| f.0).min().unwrap();
        println!("chunk_min: {m:?}");

        m
    }
}

impl Mapping {
    fn apply(&self, from: u64, to: u64) -> Vec<(u64, u64)> {
        let mut out = vec![];
        let mut ranges = vec![(from, to)];
        while let Some((from, to)) = ranges.pop() {
            if self.transforms.iter().all(|t| {
                let s = from.max(t.from);
                let e = to.min(t.from + t.len);
                if s < e {
                    out.push((t.apply(s), t.apply(e)));
                    if from < s {
                        ranges.push((from, s));
                    }
                    if e < to {
                        ranges.push((e, to));
                    }
                    return false;
                }
                true
            }) {
                out.push((from, to));
            }
        }
        out
    }
}

impl Transform {
    fn apply(&self, seed: u64) -> u64 {
        self.to + (seed - self.from)
    }
}

fn main() {
    let file = File::open("../../inputs/05.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input).unwrap();

    run(EXAMPLE);
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
            len: arr[2],
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
