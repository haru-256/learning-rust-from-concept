use myfileio::utils;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("hello.txt")?;
    let f = BufReader::new(f);

    let mut lines_vec = Vec::new();

    for line in f.lines() {
        match line {
            Ok(l) => lines_vec.push(l),
            Err(e) => return Err(e),
        }
    }

    let f = File::open("hello.txt")?;
    let mut f = BufReader::new(f);
    loop {
        let mut ll = String::new();
        let read_size = f.read_line(&mut ll)?;
        if read_size == 0 {
            break;
        }
        lines_vec.push(ll.trim_end().to_owned());
    }
    println!("{:?}", lines_vec);

    let f = File::open("hello.txt")?;
    let lines_vec = utils::line_read::get_lines(f)?;
    println!("{:?}", lines_vec);

    Ok(())
}
