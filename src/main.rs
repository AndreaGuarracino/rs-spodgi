use std::env;
use std::process;

use clap::{Arg, App};


fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let matches = App::new("gfa2rdf")
        .version(VERSION)
        .author("Andrea Guarracino")
        .about("GFA to RDF converter")
        .arg(
            Arg::with_name("input")
                .short("g")
                .long("gfa")
                .value_name("FILE")
                .help("GFA input file to convert")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let path_input_gfa = matches
        .value_of("input")
        .expect("Could not parse GFA input file");


    if let Ok(lines) = rs_spodgi::read_lines(path_input_gfa) {
        rs_spodgi::write_lines(lines).unwrap_or_else(|err| {
            println!("Problem parsing the input file: {}", err);
            process::exit(1);
        });
    }
}
