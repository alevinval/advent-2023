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
