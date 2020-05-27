use clap::{Arg,App,SubCommand};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
mod algorithm;
use algorithm::gzip;
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
            let file_path = Path::new(&file_name);
            let target_path = Path::new("target.tar");

            gzip::compress(file_path, target_path).unwrap();
            println!("Jojo");
        },
        ("extract", Some(m)) => {
            let file_name = m.value_of("INPUT").unwrap();
            let file_path = Path::new(&file_name);
            let target_path = Path::new("Target");
            gzip::decompress(file_path, target_path).unwrap();
        },
        _ => println!("Error")
    }
}
