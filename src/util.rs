pub fn get_payloads(local_file: String) -> Vec<String> {
    let payloads:Vec<String> = Vec<String>::new();
    if let Ok(lines) = read_line(local_file) {
        for line in lines {
            if let Ok(pl) = line {
                payloads.push(pl)
            }
        }
    }
    payloads
}

fn read_line<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
