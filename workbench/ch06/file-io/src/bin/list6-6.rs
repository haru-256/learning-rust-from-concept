use std::fs::File;
use std::io::{self, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[allow(non_snake_case)]
#[derive(Debug)]
struct BmpFileHeader {
    bfType: [u8; 2],
    bfSize: u32,
    bfReserved1: u16,
    bfReserved2: u16,
    bfOffBits: u32,
}

#[allow(non_snake_case)]
impl BmpFileHeader {
    fn parse_file_header(buf: &[u8]) -> std::io::Result<Self> {
        let mut cursor = Cursor::new(buf);
        let mut bfType = [0_u8; 2];
        for cc in &mut bfType {
            *cc = cursor.read_u8()?;
        }

        let bfSize = cursor.read_u32::<LittleEndian>()?;
        let bfReserved1 = cursor.read_u16::<LittleEndian>()?;
        let bfReserved2 = cursor.read_u16::<LittleEndian>()?;
        let bfOffBits = cursor.read_u32::<LittleEndian>()?;

        Ok(BmpFileHeader {
            bfType,
            bfSize,
            bfReserved1,
            bfReserved2,
            bfOffBits,
        })
    }
}

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    print!("Please input file name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    println!("Reading file: {}", input);
    let mut f = File::open(input)?;
    let mut buf_file_header = [0_u8; 14];
    let _ = f.read(&mut buf_file_header)?;

    let file_header = BmpFileHeader::parse_file_header(&buf_file_header)?;

    println!("{:?}", file_header);

    Ok(())
}
