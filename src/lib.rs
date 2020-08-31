use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("just one argument is accepted");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// From https://gist.github.com/JervenBolleman/856935510bb250991224acc7040cb5de
pub fn write_lines(lines: io::Lines<io::BufReader<File>>) -> Result<&'static str, &'static str> {
    let mut position_in_path: u64;
    let mut next_position_in_path: u64;
    let mut link_orientation: &str;
    let mut node_to_length: HashMap<String, u64> = HashMap::new();

    let mut node;
    let mut orientation;

    let mut node_term;
    let mut node_orientation;
    let mut node_orientation_long;

    let mut beg;
    let mut end;

    println!("@prefix faldo: <http://biohackathon.org/resource/faldo#> .");
    println!("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .");
    println!("@prefix vg: <http://biohackathon.org/resource/vg#> .");
    println!("@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .");
    println!("@prefix node: <http://example.org> .");

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let tokens: Vec<&str> = ip.split("\t").collect();

            match tokens[0] {
                "S" => {
                    println!("node:{} a vg:Node ;\n\trdf:value \"{}\" .", tokens[1], tokens[2]);

                    node_to_length.insert(tokens[1].parse().unwrap(), tokens[2].len() as u64);
                }
                "L" => {
                    link_orientation = "ForwardToForward";

                    if tokens[2] == "+" && tokens[4] == "-" {
                        link_orientation = "ForwardToReverse";
                    } else if tokens[2] == "-" && tokens[4] == "+" {
                        link_orientation = "ReverseToForward";
                    } else if tokens[2] == "-" && tokens[4] == "-" {
                        link_orientation = "ReverseToReverse";
                    } else if !(tokens[2] == "+" && tokens[4] == "+") {
                        return Err("the link orientations are not valid");
                    }

                    println!("node:{}\n\tvg:links{} node:{} .", tokens[1], link_orientation, tokens[3]);
                }
                "P" => {
                    println!("<{}> a vg:Path .", tokens[1]);

                    position_in_path = 0;

                    for (step_position, step) in tokens[2].split(",").enumerate() {
                        println!("\t<{}-step-{}> a vg:Step,\n\t\tfaldo:Region ;\n\t\tvg:rank {} ;\n\t\tvg:path <{}> ;", tokens[1], step_position + 1, step_position + 1, tokens[1]);

                        println!("{} {}", step, step.len());
                        node = &step[..(step.len() - 1)];
                        orientation = &step[(step.len() - 1)..];

                        node_term = "node";
                        node_orientation = "f";
                        node_orientation_long = "Forward";

                        if orientation == "-" {
                            node_term = "reverseOfNode";
                            node_orientation = "r";
                            node_orientation_long = "Reverse";
                        } else if orientation != "+" {
                            return Err("the node orientation is not valid");
                        }

                        next_position_in_path = position_in_path + node_to_length.get(node).unwrap();

                        beg = tokens[1].to_owned() + node_orientation + position_in_path.to_string().as_str();
                        end = tokens[1].to_owned() + node_orientation + next_position_in_path.to_string().as_str();

                        println!("\t\tvg:{} node:{} ;\n\t\tfaldo:begin <{}> ;\n\t\tfaldo:end <{}> . ", node_term, node, beg, end);
                        println!("\t\t<{}> a faldo:ExactPosition,\n\t\t\tfaldo:{}StrandPosition ;\n\t\t\tfaldo:position {} ;\n\t\t\tfaldo:reference <{}> . ", beg, node_orientation_long, position_in_path, tokens[1]);
                        println!("\t\t<{}> a faldo:ExactPosition,\n\t\t\tfaldo:{}StrandPosition ;\n\t\t\tfaldo:position {} ;\n\t\t\tfaldo:reference <{}> . ", end, node_orientation_long, next_position_in_path, tokens[1]);

                        position_in_path = next_position_in_path;
                    }
                }
                "H" => (), // Header
                _ => {
                    return Err("the input file is not a valid GFA");
                }
            };
        }
    }

    return Ok("Done");
}