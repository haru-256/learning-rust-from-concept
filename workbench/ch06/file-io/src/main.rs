mod utils;

use std::fs::File;
use std::io::{self};

fn main() -> io::Result<()> {
    let f = File::open("hello.txt")?;
    let lines_vec = utils::line_read::get_lines(f)?;
    println!("{:?}", lines_vec);
    Ok(())
}
