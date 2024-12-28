use std::fs::File;
use std::io::BufReader;

/*
- Open file
- Read headers and frame info
- Iterate by chunks (frame-size)
*/

pub fn open(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input)?;
    let buf_reader = BufReader::new(file);
    
    Ok(())
}