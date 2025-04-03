use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    let mut fr = File::open("inputs/input.txt")?;
    let mut fw = OpenOptions::new()
        .append(true)
        .create(false)
        .open("inputs/output.txt")?;

    let mut buffer = [0_u8; BUFFER_SIZE];

    loop {
        let read_size = fr.read(&mut buffer)?;
        if read_size == 0 {
            break;
        } else {
            // write_all is better way, which writes all data at once
            // fw.write_all(&buffer[..read_size])?;
            // write writes data in chunks but it is not guaranteed that all data will be written
            let written_size = fw.write(&buffer[..read_size])?;
            if written_size != read_size {
                panic!("Error writing to file");
            }
        }
    }

    Ok(())
}
