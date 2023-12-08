use std::collections::HashMap;

static EXAMPLE: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

static PART2_EXAMPLE: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

#[derive(Debug)]
enum D {
    L,
    R,
}

#[derive(Debug)]
struct Entry {
    from: String,
    left: String,
    right: String,
}

fn solve(d: Vec<D>, entries: Vec<Entry>, part2: bool) -> usize {
    let starting_nodes: Vec<&str> = if part2 {
        entries
            .iter()
            .filter(|e| e.from.ends_with('A'))
            .map(|e| e.from.as_str())
            .collect()
    } else {
        vec!["AAA"]
    };

    let mapping: HashMap<&str, &Entry> = entries.iter().fold(HashMap::new(), |mut acc, entry| {
        acc.insert(entry.from.as_str(), entry);
        acc
    });

    let find_z = |input: &str, mappings: &HashMap<&str, &Entry>| -> usize {
        let mut curr = input;
        let mut nn = 0;
        while !curr.ends_with('Z') {
            for nd in &d {
                curr = match nd {
                    D::L => &mappings[curr].left,
                    D::R => &mappings[curr].right,
                }
            }
            nn += 1;
        }
        nn
    };

    starting_nodes
        .iter()
        .map(|node| find_z(node, &mapping))
        .reduce(advent::lcm)
        .unwrap()
        * d.len()
}

fn run(input: &str, part2: bool) {
    let mut lines = input.lines();
    let flow: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'R' { D::R } else { D::L })
        .collect();
    lines.next();
    let map: Vec<Entry> = lines
        .enumerate()
        .map(|(_, l)| {
            let s: Vec<_> = l.split('=').collect();
            let d: Vec<_> = s[1]
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(' ')
                .collect();
            Entry {
                from: s[0].trim().to_string(),
                left: d[0].strip_suffix(',').unwrap().to_string(),
                right: d[1].to_string(),
            }
        })
        .collect();

    let ans = solve(flow, map, part2);
    println!("{ans}");
}

fn main() {
    let input = advent::open("08");

    println!("PART ONE");
    run(EXAMPLE, false);
    run(&input, false);

    println!("PART TWO");
    run(PART2_EXAMPLE, true);
    run(&input, true);
}
