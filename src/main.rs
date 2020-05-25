use clap::{Arg,App,SubCommand};
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
        ("compress", _) => {println!("Compress")},
        ("extract", _) => {println!("Extract")},
        _ => println!("Error")
    }
}
