use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::SeekFrom;

pub fn initialize_champernowne(range: i64) -> String {
    /// # initialize_champernowne(range: i64) -> String
    /// Creates a text file which stores the values of champernowne up until a range
    /// ## Input
    /// Input a range (max) (inclusive) up to the 64 bit integer limit 
    /// |_ I dont reccomend generating over 100 billion digits or range=~10200000000
    /// ## Output
    /// Outputs the string "Done" and the src/bin/champernowne.txt
    
    // Define the string value
    let mut constant: String = String::new();
    for i in 1..=range {
        // Append the i value to the string (concatenate)
        write!(&mut constant, "{}", i).unwrap();
    }

    //Writes to the file
    std::fs::write("src/bin/champernowne.txt", constant).expect("Unable to write file");

    //Returns once the function is done
    return String::from("Done");
}

pub fn read_champernowne(filename: &str, n: usize) -> Result<char, Error> {
    /// # read_champernowne(filename: &str, n: usize) -> Result<char, Error>
    /// Reads from the text file which stores the values of champernowne and returns the n value
    /// ## Input
    /// Input a filename from which the function reads
    /// Input a nth digit which will be printed
    /// ## Output
    /// RETURNS the status code and the byte (character)

    //Opens fine
    let mut file = File::open(filename)?;

    //Seeks through the file until the nth digit
    file.seek(SeekFrom::Start(n as u64))?;

    //Reads the value and returns
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
