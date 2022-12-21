//! Little_helpers is a library of helper functions intended to be used for working with Advent of Code data.


mod test;
pub mod listify {

    use std::fs::{File, read_to_string};
    use std::io::{Write, Result};

    /// Copies the contents of one file into a new file, with one item per line.
    /// 
    /// The items from the original file will be split at the delimiter token, and empty lines will be ignored.
    /// Returns the File handle of the new file, or an error if it fails to create the output file.
    pub fn listify_into_file(file_in: String, file_out: &str, delimiter: &str) -> Result<File> {

        let file_contents = read_to_string(file_in)?;
        let mut output_file = File::create(&file_out)?;
        for line in file_contents.split(delimiter).filter(|s| !s.is_empty()) {
            writeln!(output_file, "{}", line)?;
        }
        Ok(output_file)
    }

    /// Returns a Result<Vec<T>> from a file using a delimiter token.
    /// 
    /// Output vector only contains value types implementing the FromStr trait.
    /// If parsing to something that is not a string type, invalid values will be omitted
    pub fn listify_into_vec<T: std::str::FromStr>(file_in: &str, delimiter: &str) -> Result<Vec<T>>
    {
        let input_file = read_to_string(file_in)?;

        let file_contents: Vec<T> = input_file.split(delimiter).filter_map(|val| val.parse().ok()).collect();
        Ok(file_contents)
    }

    /// Returns a Result<Vec<Option<T>>> from a file using a lines as delimiter token
    /// 
    /// Output vector only contains value types implementing the FromStr trait.
    /// If parsing to something that is not a string type, invalid lines will be None
    pub fn listify_lines<T: std::str::FromStr>(file_in: &str) -> Result<Vec<Option<T>>> {
        let input_file = read_to_string(file_in)?;

        let file_contents: Vec<Option<T>> = input_file.lines().map(|val| val.parse().ok()).collect();
        Ok(file_contents)

    }
}

pub mod coords {
    
    /// Maps 2d coordinates to a 1d index based on the width of the coordinate system
    /// Returns usize
    pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize
    {
        x + (y * width)
    }

    /// Calculates a 1d index into a 2d X-coordinate
    pub fn index_to_x(index: usize, width: usize) -> usize
    {
        index % width
    }

    /// Calculates a 1d index into a 2d Y-coordinate
    pub fn index_to_y(index: usize, width: usize) -> usize
    {
        index / width
    }

    /// Checks if a given coordinate is along an edge
    pub fn coord_along_edge(x: usize, y: usize, width: usize, height: usize) -> bool
    {
        if  x == 0 ||
            x == width - 1 || 
            y == 0 ||
            y == height - 1 
        { true } 
        else
        { false }
    }

    /// Checks if a given index is along an edge
    pub fn index_along_edge(index: usize, width: usize, height: usize) -> bool
    {
        if  index_to_x(index, width) == 0 ||
            index_to_x(index, width) == width - 1 ||
            index_to_y(index, width) == 0 ||
            index_to_y(index, width) == height - 1
        { true }
        else 
        { false }
    }

}
