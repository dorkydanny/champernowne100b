use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::SeekFrom;

pub fn initialize_champernowne(range: i64) -> String {
    let mut constant: String = String::new();
    for i in 1..=range {
        write!(&mut constant, "{}", i).unwrap();
    }
    std::fs::write("src/bin/champernowne100k.txt", constant).expect("Unable to write file");
    return String::from("Done");
}

pub fn read_champernowne(filename: &str, n: usize) -> Result<char, Error> {
    let mut file = File::open(filename)?;
    file.seek(SeekFrom::Start(n as u64))?;

    let mut byte = [0; 1];
    file.read_exact(&mut byte)?;

    Ok(byte[0] as char)
}

fn main() {
    let mut n = 1;
    for _ in 1..=7 {
        match read_champernowne("src/bin/champernowne1m.txt", n - 1) {
            Ok(c) => println!("The character at index {} is: {}", n, c),
            Err(e) => println!("Error reading file: {}", e),
        }
        n = (n * 10);
    }
}
