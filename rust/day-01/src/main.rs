use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read},
};

fn main() -> io::Result<()> {
    let file = File::open("../../inputs/01.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut dict = HashMap::new();
    dict.insert("one", "1");
    dict.insert("two", "2");
    dict.insert("three", "3");
    dict.insert("four", "4");
    dict.insert("five", "5");
    dict.insert("six", "6");
    dict.insert("seven", "7");
    dict.insert("eight", "8");
    dict.insert("nine", "9");

    let mut overlaps = HashMap::new();
    overlaps.insert("oneight", "18");
    overlaps.insert("twone", "21");
    overlaps.insert("threeight", "38");
    overlaps.insert("fiveight", "58");
    overlaps.insert("eightwo", "82");
    overlaps.insert("eightree", "83");

    let mut result = 0;
    for line in contents.lines() {
        let line = normalize(&dict, &overlaps, line.into());
        result += recover_code(&line)
    }

    println!("Result is {}", result);
    return Ok(());
}

fn normalize(
    dict: &HashMap<&str, &str>,
    overrides: &HashMap<&str, &str>,
    mut line: String,
) -> String {
    overrides
        .iter()
        .chain(dict)
        .for_each(|(from, to)| line = line.replace(from, to));
    line
}

fn recover_code(line: &str) -> u32 {
    let (idx, first) = line
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_numeric())
        .next()
        .unwrap();
    let second = line
        .chars()
        .rev()
        .take(line.len() - idx)
        .filter(|c| c.is_numeric())
        .next()
        .unwrap();
    return first.to_digit(10).unwrap() * 10 + second.to_digit(10).unwrap();
}
