use std::fs::File;
use std::io::Read;

pub fn open(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(input)?;

    let mut header = [0u8; 12];
    let bytes_read = file.read_exact(&mut header)?;

    let riff = &header[0..4];
    let avi_signature = &header[8..12];
    if  !(riff == b"RIFF" && avi_signature == b"AVI ") {
        return Err("File signature does not match an AVI file".to_string().into());
    }

    let file_size = u32::from_le_bytes(header[4..8].try_into()?);
    Ok(())
}