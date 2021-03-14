use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};

fn inspect(content: &Vec<u8>, full_string: bool) {
    if full_string {
        println!("{:?}", content);
        println!("{}", String::from_utf8_lossy(content));
    } else {
        let mut i = 0;
        for x in content {
            i += 1;
            println!("{}, {}, {}", *x, *x as char, i);
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("./tt067t0")?;
    let mut reader = BufReader::new(f);

    let mut line: Vec<u8> = Vec::new();

    let len = reader.read_to_end(&mut line)?;

    let cursor = io::Cursor::new(&line);
    let mut split_iter = cursor.split(0u8).map(|l| l.unwrap());

    while let Some(mut bytes) = split_iter.next() {
        //TODO remove_rdw
        //remove_at_signs(bytes);
        //remove_slice(&mut bytes, &vec![b'@', b'@']) ;
        bytes.retain(|byte| *byte != b'@');

        if bytes.len() > 0 {
            inspect(&bytes, true);
        }
    }

    // TODO write the header extractor
    // TODO extract it
    println!("First line is {} bytes long", len);

    Ok(())
}
