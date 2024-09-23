use std::io::{self, BufRead, BufReader, Read};

pub fn get_lines<T: Read>(f: T) -> io::Result<Vec<String>> {
    let f = BufReader::new(f);
    let mut lines_vec = Vec::new();
    for ll in f.lines() {
        match ll {
            Ok(l) => lines_vec.push(l),
            Err(e) => return Err(e),
        }
    }
    Ok(lines_vec)
}
