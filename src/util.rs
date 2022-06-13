use std::io::BufRead;
use std::{io};
use std::path::Path;
use std::fs::File;

pub fn get_payloads(local_file: &str) -> Option<Vec<String>> {
    let mut payloads:Vec<String> = Vec::new();
    if let Ok(lines) = read_line(local_file) {
        for line in lines {
            if let Ok(pl) = line {
                payloads.push(pl);
            }
        }
    }
    Some(payloads)
}


fn read_line<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
