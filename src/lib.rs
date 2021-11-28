//! Little_helpers is a library of helper functions intended to be used for reading Advent of Code input data.

use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Write};

/// Returns a vector of values by line from a file.
///
///As every line is a string, the function restricts conversions to any type the FromStr trait implements.
/// # Example
/// ```rust
/// // Assuming a file "input.txt" is located in the working directory
///
/// // Returns a vector of values parsed to i32
/// let vector: Vec<i32> = read_file("input.txt");
///
/// // Returns a vector of values parsed to String
/// let vector2: Vec<String> = read_file("input.txt");
/// ```
pub fn read_file<T: std::str::FromStr>(file: String) -> Vec<T> {
    let input_file = File::open(file).expect("Could not open file!");
    let buf_reader = BufReader::new(input_file);
    let mut file_contents: Vec<T> = Vec::new();

    let mut i = 0;
    for line in buf_reader.lines() {
        let parsed = match line.unwrap().parse::<T>() {
            Ok(generic_type) => {generic_type},
            Err(_e) => {panic!("Something went wrong when processing line {}", i);},
        };
        file_contents.push(parsed);
        i += 1;
    }
    file_contents
}

/// Creates a new file containing the data from the input file where each item is on a separate line.
/// Items will be separated at the provided delimiter char.
///
/// Returns the file name of the new file
/// # Example
/// ```rust
/// // Assuming a file "input.txt" is located in the working directory
/// /* "input.txt"  > 1,2,3
///    "output.txt" >   1
///                     2
///                     3
/// */
/// let output_file_name: String = listify("input.txt", "output.txt", ',');
///
/// assert_eq!(output_file_name, "output.txt");
/// ```
pub fn listify(file_in: &str, file_out: &str, delimiter: char) -> String {

    let file_contents = read_to_string(file_in).expect("Could not open file!");
    let mut output_file = File::create(format!("{}", file_out)).unwrap();

    let mut i = 0;
    for line in file_contents.split(delimiter) {
        writeln!(output_file, "{}", line).expect(&format!("Could not write line number {} to file!", i));
        i += 1;
    }
    drop(i);

    String::from(file_out)
}