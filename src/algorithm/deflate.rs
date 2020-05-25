use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

pub fn compress(src: &mut impl Read,target: impl Write) -> io::Result<()> {
    let mut data: Vec<u8> = vec![];
    src.read_to_end(&mut data)?;
    let mut compressor = DeflateEncoder::new(target, Compression::default());
    compressor.write_all(&data)
}