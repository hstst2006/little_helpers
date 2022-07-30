//! Little_helpers is a library of helper functions intended to be used for reading Advent of Code input data.

use std::fs::{File, read_to_string};
use std::io::Write;

/// Creates a new file containing the data from the input file where each item is separated by newlines.
/// Items will be separated at the provided delimiter char.
///
/// Returns the file name of the new file, or an error if failing to write a line to file
/// # Example
/// ```rust
/// // Assuming a file "input.txt" is located in the working directory
/// /* "input.txt"  > 1,2,3
///    "output.txt" >   1
///                     2
///                     3
/// */
/// ```
pub fn listify_into_file(file_in: String, file_out: String, delimiter: char) ->  Result<String, std::io::Error> {

    let file_contents = read_to_string(file_in).expect("Could not open file!");
    let mut output_file = File::create(format!("{}", file_out)).unwrap();

    for line in file_contents.split(delimiter) {
        if !line.is_empty() {
            match writeln!(output_file, "{}", line) {
                Ok(_) => {},
                Err(e) => {return Err(e);}
            };
        }
    }

    Ok(String::from(file_out))
}

/// Returns a vector of values from a file using a delimiter char.
///
/// As every item in the text file is a string, the function restricts conversions to any type the FromStr trait implements.
pub fn listify_into_vec<T: std::str::FromStr>(file_in: String, delimiter: char) -> Vec<T> {
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