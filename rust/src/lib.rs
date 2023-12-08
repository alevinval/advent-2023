use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn open(day: &str) -> String {
    let file = File::open(format!("../inputs/{day}.txt")).expect("cannot open input");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("cannot read input");
    contents
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn gcd(mut min: usize, mut max: usize) -> usize {
    if min > max {
        (max, min) = (min, max);
    }
    loop {
        let r = max % min;
        if r == 0 {
            return min;
        }

        max = min;
        min = r;
    }
}
