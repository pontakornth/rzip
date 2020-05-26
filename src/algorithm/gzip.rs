use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use tar::{Builder, Archive};

pub fn compress(src_path: &str,target_path: &str) -> io::Result<()> {
    let mut src = File::open(src_path)?;
    let mut temp_target = File::create("temp")?;
    let mut data: Vec<u8> = vec![];
    {
        let mut builder = Builder::new(&mut temp_target);
        builder.append_file(src_path, &mut src)?;
        builder.finish()?;
    }
    let target = File::create(target_path)?;
    let mut temp_target = File::open("temp")?;
    temp_target.read_to_end(&mut data)?;
    let mut compressor = GzEncoder::new(target, Compression::default());
    compressor.write_all(&data)
}

pub fn decompress(src_path: &str,target_path: &str) -> io::Result<()> {
    let mut src = File::open(src_path)?;
    let temp_target = File::create("temp.tar")?;
    let mut data: Vec<u8> = vec![];
    {
        src.read_to_end(&mut data)?;
        let mut decompressor = GzDecoder::new(temp_target);
        decompressor.write_all(&data)?;
    }
    let temp_target = File::open("temp.tar")?;
    let mut archive = Archive::new(temp_target);
    archive.unpack(target_path)
}