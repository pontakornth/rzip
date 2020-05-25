use clap::{Arg,App,SubCommand};
use std::io::prelude::*;
use std::fs::File;
mod algorithm;
use algorithm::deflate;
fn main() {
    let matches = App::new("R-Zip")
        .version("0.0.1")
        .author("Pontakon Paesaeng <pontakorn_most@outlook.com>")
        .about("Zipping and unzipping with Rust")
        .subcommand(SubCommand::with_name("compress")
                    .about("Compress")
                    .arg(Arg::with_name("INPUT")
                        .required(true)
                        .takes_value(true)
                    )
        )
        .subcommand(SubCommand::with_name("extract")
                    .about("Extract")
                    .arg(Arg::with_name("INPUT")
                        .required(true)
                        .takes_value(true)
                    )
        )
        .get_matches();
    match matches.subcommand() {
        ("compress", Some(m)) => {
            let file_name = m.value_of("INPUT").unwrap();
            let mut src = File::open(file_name).unwrap();
            let target = File::create([file_name, "_compressed"].join("")).unwrap();
            deflate::compress(&mut src, &target).unwrap();
        },
        ("extract", Some(m)) => {
            let file_name = m.value_of("INPUT").unwrap();
            let mut src = File::open(file_name).unwrap();
            let target = File::create([file_name, "_decompressed"].join("")).unwrap();
            deflate::decompress(&mut src, &target).unwrap();
        },
        _ => println!("Error")
    }
}
