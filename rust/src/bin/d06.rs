static EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

fn run(input: &str) {
    let mut lines = input.lines();
    let times: Vec<_> = lines.next().expect("first line")[6..]
        .split(" ")
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| u64::from_str_radix(t, 10).expect(&format!("should be number: '{t}'")))
        .collect();

    let distances: Vec<_> = lines.next().expect("second line")[11..]
        .split(" ")
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| u64::from_str_radix(t, 10).expect("should be number"))
        .collect();

    // Part 1
    let races: Vec<_> = times.iter().zip(&distances).collect();
    let results: Vec<_> = races.iter().map(|(t, d)| solve(**t, **d)).collect();
    let ans = results.into_iter().reduce(|a, b| a * b).unwrap();
    println!("part 1: {ans}");

    // Part 2
    let time = u64::from_str_radix(
        &times
            .iter()
            .map(|t| t.to_string())
            .reduce(|a, b| a + &b)
            .unwrap(),
        10,
    )
    .unwrap();

    let distance = u64::from_str_radix(
        &distances
            .iter()
            .map(|t| t.to_string())
            .reduce(|a, b| a + &b)
            .unwrap(),
        10,
    )
    .unwrap();

    let ans = solve(time, distance);
    println!("part 2: {ans}")
}

fn solve(time: u64, distance: u64) -> u64 {
    let mut press = 1;
    let mut wins = 0;

    while press < time {
        if press * (time - press) > distance {
            wins += 1;
        } else if wins > 0 {
            break;
        }
        press += 1;
    }

    wins
}

fn main() {
    println!("example");
    run(EXAMPLE);

    println!("input");
    let input = advent::open("06");
    run(&input);
}
