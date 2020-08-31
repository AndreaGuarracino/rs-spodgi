use std::env;
use std::process;


use rs_spodgi::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Ok(lines) = rs_spodgi::read_lines(config.filename) {
        rs_spodgi::write_lines(lines).unwrap_or_else(|err| {
            println!("Problem parsing the input file: {}", err);
            process::exit(1);
        });
    }
}
