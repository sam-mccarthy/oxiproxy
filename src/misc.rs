use std::{fs, io};

pub fn load_file(filename: &str) -> io::BufReader<fs::File> {
    let file = fs::File::open(filename)
        .map_err(|e| error(
            format!("Failed to open file {}: {}", filename, e)
        ))?;
    io::BufReader::new(file)
}

pub fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}