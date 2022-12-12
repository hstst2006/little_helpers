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
        listify_into_file("test_file.txt".to_string(), "output.txt", ",")?;

        assert_eq!("0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n", read_to_string("output.txt")?);

        cleanup_test_files("test_file.txt", Some("output.txt"))?;

        Ok(())
    }

    #[test]
    fn test_vector_i32() -> Result<()> {
        generate_test_file("test_vector.txt")?;
        let vector: Vec<i32> = listify_into_vec("test_vector.txt", ",")?;
        for n in 0..10 {
            assert_eq!(vector[n], n as i32);
        }
        cleanup_test_files("test_vector.txt", None)?;
        Ok(())
    }
}