//! Little_helpers is a library of helper functions intended to be used for reading Advent of Code input data.
use std::fs::{File, read_to_string};
use std::io::{Write, Result};

#[cfg(test)]
mod test;

/// Copies the contents of one file into a new file, with one item per line.
/// 
/// The items from the original file will be split at the delimiter token.
/// Returns the File handle of the new file, or an error if it fails to create the output file.
pub fn listify_into_file(file_in: String, file_out: String, delimiter: &str) -> Result<File> {

    let file_contents = read_to_string(file_in)?;
    let mut output_file = File::create(&file_out)?;

    for line in file_contents.split(delimiter).filter(|s| !s.is_empty()) {
        writeln!(output_file, "{}", line)?;
    }

    Ok(output_file)
}

/// Returns a vector of values from a file using a delimiter token.
/// 
/// Output vector only contains value types implementing the FromStr trait.
pub fn listify_into_vec<T: std::str::FromStr>(file_in: String, delimiter: &str) -> Result<Vec<T>> {
    let input_file = read_to_string(file_in)?;
    let file_contents: Vec<T> = input_file.split(delimiter).filter_map(|val| val.parse().ok()).collect();
    Ok(file_contents)
}