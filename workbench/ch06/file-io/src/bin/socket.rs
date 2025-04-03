use std::io;
use std::net::TcpListener;

use myfileio::utils;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3210")?;

    for stream in listener.incoming() {
        let lines_vec = utils::line_read::get_lines(stream?)?;
        println!("Received lines: {:?}", lines_vec);
    }
    Ok(())
}
