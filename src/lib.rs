//! Little_helpers is a library of helper functions intended to be used for reading Advent of Code input data.

use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, Write};

/// Returns a vector of values from a file using lines
///
///As every line is a string, the function restricts conversions to any type the FromStr trait implements.
/// # Example
/// ```rust
/// // Assuming a file "input.txt" is located in the working directory
///
/// // Returns a vector of values parsed to i32
/// let vector: Vec<i32> = read_file(String::from("input.txt"));
///
/// // Returns a vector of values parsed to String
/// let vector2: Vec<String> = read_file(String::from("input.txt"));
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
pub fn listify(file_in: String, file_out: String, delimiter: char) -> String {

    let file_contents = read_to_string(file_in).expect("Could not open file!");
    let mut output_file = File::create(format!("{}", file_out)).unwrap();

    let mut i = 0;
    for line in file_contents.split(delimiter) {
        if !line.is_empty() {
            writeln!(output_file, "{}", line).expect(&format!("Could not write line number {} to file!", i));
            i += 1;
        }
    }
    drop(i);

    String::from(file_out)
}

/// Returns a vector of values from a file using a delimiter char.
///
///As every item is a string, the function restricts conversions to any type the FromStr trait implements.
/// Conveniently lets us avoid using listify and read_file in conjunction.
///
/// # Example
/// ```rust
/// // Assuming a file "input.txt" is located in the working directory
/// // Assuming the values to be read are separated by a known delimiter
///
/// // Returns a vector of values parsed to i32
/// let vector: Vec<i32> = read_listified(String::from("input.txt"), ',');
///
/// // Returns a vector of values parsed to String
/// let vector2: Vec<String> = read_listified(String::from("input.txt"), ',');
/// ```
pub fn read_listified<T: std::str::FromStr>(file_in: String, delimiter: char) -> Vec<T> {
    let input_file = read_to_string(file_in).expect("Could not open file!");
    let mut file_contents: Vec<T> = Vec::new();

    let mut i = 0;
    for item in input_file.split(delimiter) {
        let parsed = match item.parse::<T>() {
            Ok(generic_type) => {generic_type},
            Err(_e) => {panic!("Something went wrong when processing item {}", i)},
        };
        file_contents.push(parsed);
        i += 1;
    }
    drop(i);

    file_contents
}