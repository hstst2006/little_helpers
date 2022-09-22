//! Little_helpers is a library of helper functions intended to be used for reading Advent of Code input data.

use std::fs::{File, read_to_string};
use std::io::{Write, Result};

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
pub fn listify_into_vec<T: std::str::FromStr>(file_in: String, delimiter: &str) -> Vec<T> {
    let input_file = read_to_string(file_in)?;
    let file_contents: Vec<T> = input_file.split(delimiter).filter_map(|val| val.parse().ok()).collect();
    file_contents
}

#[cfg(test)]
mod tests {

    use crate::{listify_into_file, listify_into_vec};
    use std::fs::{File, read_to_string, remove_file};
    use std::io::{Write, Result};

    fn generate_test_file(filename: &str) -> Result<()> {
        let mut file = File::create(filename).unwrap();

        for n in 0..10 {
            write!(file, "{},", n)?;
        }

        Ok(())
    }

    fn cleanup_test_files(filename: &str, filename2: Option<&str>) -> Result<()> {
        remove_file(filename)?;
        if let Some(v) = filename2 {
            remove_file(v)?;
        }

        Ok(())
    }

    #[test]
    fn test_file() -> Result<()> {
        generate_test_file("test_file.txt")?;
        listify_into_file("test_file.txt".to_string(), "output.txt".to_string(), ",")?;

        assert_eq!("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n", read_to_string("output.txt")?);

        cleanup_test_files("test_file.txt", Some("output.txt"))?;

        Ok(())
    }

    #[test]
    fn test_vector_i32() -> Result<()> {
        generate_test_file("test_vector.txt")?;
        let vector: Vec<i32> = listify_into_vec("test_vector.txt".to_string(), ",");
        for n in 0..10 {
            assert_eq!(vector[n], n as i32);
        }
        cleanup_test_files("test_vector.txt", None)?;
        Ok(())
    }
}