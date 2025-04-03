use std::fs::File;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let mut fr = File::open("inputs/input.txt")?;
    let mut fw = File::create("inputs/output.txt")?;

    let mut buffer = [0_u8; BUFFER_SIZE];

    loop {
        let read_size = fr.read(&mut buffer)?;
        if read_size == 0 {
            break;
        } else {
            fw.write_all(&buffer[..read_size])?;
        }
    }

    Ok(())
}
