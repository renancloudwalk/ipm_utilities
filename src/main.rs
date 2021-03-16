use std::convert::TryFrom;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

// Each subsequent byte has a potential value of 255 (because it's in ASCII)
// so a RDW of 0u8 0u8 1u8 3u8 actually means that the RDW refers to the next 258 characters
// (0 × 255³) + (0 × 255²) + (1 × 255¹) + (3 × 255⁰) = 258
fn rdw_to_size(rdw_buffer: &[u8]) -> usize {
    let s: u64 = rdw_buffer
        .iter()
        .enumerate()
        .map(|(index, rdw_number)| -> u64 {
            let index_translation = i8::abs(i8::try_from(index).unwrap() - 3i8);
            let index_power: u64 = 255u64.pow(u32::try_from(index_translation).unwrap());
            u64::try_from(*rdw_number).unwrap() * index_power
        })
        .sum();
    usize::try_from(s).unwrap()
}

fn main() -> std::io::Result<()> {
    let f = File::open("./tt067t0")?;

    // removing at signs
    let mut big_line: Vec<u8> = f.bytes().map(|x| x.unwrap()).collect();
    big_line.retain(|byte| *byte != b'@');

    let mut position: usize = 0;
    while big_line.len() > position {
        //checking for EOF
        if &big_line[position] != &0u8 {
            position += 1;
        };

        let rdw_slice = &big_line[position..position + 4];
        let calculated_rdw = rdw_to_size(&rdw_slice);

        position = position + 4;

        let content_slice = &big_line[position..(position + calculated_rdw)];
        println!("{}", String::from_utf8_lossy(content_slice));
        position = position + calculated_rdw;
    }
    Ok(())
}
