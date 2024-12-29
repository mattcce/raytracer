use std::fs::File;
use std::io::Write;

pub fn write_to_file(
    output_string: &str,
    path_string: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::create(path_string)?;
    f.write(output_string.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn write_file() {
        let output_string = "hello, world!";
        let path_string = "test.txt";
        write_to_file(output_string, path_string).unwrap();

        let mut text = String::new();
        File::open(path_string)
            .unwrap()
            .read_to_string(&mut text)
            .unwrap();
        assert_eq!(output_string, text);

        // cleanup
        std::fs::remove_file(path_string).unwrap();
    }
}
