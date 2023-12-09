use std::iter::zip;

static EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

fn solve(input: Vec<i64>, part2: bool) -> i64 {
    let mut levels = vec![input];
    while levels[levels.len() - 1].iter().sum::<i64>() != 0 {
        let level = &levels[levels.len() - 1];
        let s: Vec<_> = zip(level.iter(), level.iter().skip(1))
            .map(|(a, b)| *b - *a)
            .collect();
        levels.push(s);
    }

    if part2 {
        let mut last = 0;
        levels.iter().rev().for_each(|level| last = level[0] - last);
        last
    } else {
        levels
            .iter()
            .rev()
            .map(|level| level[level.len() - 1])
            .sum()
    }
}

fn run(input: &str, part2: bool) {
    let ans: i64 = input
        .lines()
        .map(|x| x.split(' ').map(|n| n.parse().unwrap()).collect())
        .map(|x| solve(x, part2))
        .sum();
    println!("{ans}");
}

fn main() {
    let input = advent::open("09");

    println!("PART ONE");
    run(EXAMPLE, false);
    run(&input, false);

    println!("PART TWO");
    run(EXAMPLE, true);
    run(&input, true);
}
