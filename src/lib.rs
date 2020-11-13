use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use rdf::specs::xml_specs::XmlDataTypes;

use rdf::writer::turtle_writer::TurtleWriter;
//use rdf::writer::rdf_writer::RdfWriter; // Necessary to be able to call 'writer.write_to_string(...)'

use rdf::graph::Graph;
use rdf::uri::Uri;
use rdf::triple::Triple;
use rdf::namespace::Namespace;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// This implementation has been inspired by https://gist.github.com/JervenBolleman/856935510bb250991224acc7040cb5de
pub fn write_lines(lines: io::Lines<io::BufReader<File>>) -> Result<&'static str, &'static str> {
    let mut position_in_path: u64;
    let mut next_position_in_path: u64;
    let mut node_to_length: HashMap<String, u64> = HashMap::new();

    let prefix_faldo = &Namespace::new(
        "faldo".to_string(),
        Uri::new("http://biohackathon.org/resource/faldo#".to_string()),
    );
    let prefix_rdf = &Namespace::new(
        "rdf".to_string(),
        Uri::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()),
    );
    let prefix_vg = &Namespace::new(
        "vg".to_string(),
        Uri::new("http://biohackathon.org/resource/vg#".to_string()),
    );
    let prefix_xsd = &Namespace::new(
        "xsd".to_string(),
        Uri::new("http://www.w3.org/2001/XMLSchema#".to_string()),
    );
    let prefix_node = &Namespace::new(
        "node".to_string(),
        Uri::new("http://example.org/vg/node/".to_string()),
    );
    let prefix_path = &Namespace::new(
        "path".to_string(),
        Uri::new("http://example.org/vg/path/".to_string()),
    );

    let mut graph = Graph::new(None);
    graph.add_namespace(prefix_faldo);
    graph.add_namespace(prefix_rdf);
    graph.add_namespace(prefix_vg);
    graph.add_namespace(prefix_xsd);
    graph.add_namespace(prefix_node);
    graph.add_namespace(prefix_path);

    let a = graph.create_uri_node_from_namespace_and_id(
        prefix_rdf, "type",
    );

    let writer = TurtleWriter::new(graph.namespaces());

    print!("{}", writer.write_base_uri(&graph));
    print!("{}", writer.write_prefixes(&graph));

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            let tokens: Vec<&str> = ip.split("\t").collect();

            let mut triples: Vec<Triple> = vec![];

            match tokens[0] {
                "S" => {
                    //println!("node:{} a vg:Node ;\n\trdf:value \"{}\" .", tokens[1], tokens[2]);
                    let subject = graph.create_uri_node_from_namespace_and_id(
                        prefix_node, tokens[1],
                    );

                    triples.push(Triple::new(
                        &subject,
                        &a,
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_vg, "Node",
                        ),
                    ));

                    triples.push(Triple::new(
                        &subject,
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_rdf, "value",
                        ),
                        &graph.create_literal_node(
                            tokens[2].to_string()
                        ),
                    ));

                    node_to_length.insert(tokens[1].parse().unwrap(), tokens[2].len() as u64);
                }
                "L" => {
                    let mut link_orientation = "linksForwardToForward";

                    if tokens[2] == "+" && tokens[4] == "-" {
                        link_orientation = "linksForwardToReverse";
                    } else if tokens[2] == "-" && tokens[4] == "+" {
                        link_orientation = "linksReverseToForward";
                    } else if tokens[2] == "-" && tokens[4] == "-" {
                        link_orientation = "linksReverseToReverse";
                    } else if !(tokens[2] == "+" && tokens[4] == "+") {
                        return Err("the link orientations are not valid");
                    }

                    //println!("node:{}\n\tvg:{} node:{} .", tokens[1], link_orientation, tokens[3]);
                    triples.push(Triple::new(
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_node, tokens[1],
                        ),
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_vg, link_orientation,
                        ),
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_node, tokens[3],
                        ),
                    ));
                }
                "P" => {
                    let mut node;
                    let mut orientation;

                    let mut node_term;
                    let mut node_orientation;
                    let mut node_orientation_long;

                    let mut beg;
                    let mut end;

                    //println!("<{}> a vg:Path .", tokens[1]);
                    triples.push(Triple::new(
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_path, tokens[1],
                        ),
                        &a,
                        &graph.create_uri_node_from_namespace_and_id(
                            prefix_vg, "Path",
                        ),
                    ));

                    position_in_path = 0;

                    for (step_position, step) in tokens[2].split(",").enumerate() {
                        if !step.is_empty() {
                            //println!("\t<{}-step-{}> a vg:Step,\n\t\tfaldo:Region ;\n\t\tvg:rank {} ;\n\t\tvg:path <{}> ;", tokens[1], step_position + 1, step_position + 1, tokens[1]);
                            let mut uri_string = "".to_string();
                            uri_string.push_str(tokens[1]);
                            uri_string.push_str("-");
                            uri_string.push_str("region");
                            uri_string.push_str("-");
                            uri_string.push_str(&*format!("{}", step_position + 1));

                            let subject = graph.create_uri_node(&Uri::new(uri_string));

                            triples.push(Triple::new(
                                &subject,
                                &a,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_vg, "Step",
                                ),
                            ));
                            triples.push(Triple::new(
                                &subject,
                                &a,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "Region",
                                ),
                            ));
                            triples.push(Triple::new(
                                &subject,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_vg, "rank",
                                ),
                                &graph.create_literal_node_with_data_type(
                                    (step_position + 1).to_string(),
                                    &XmlDataTypes::Integer.to_uri()
                                ),
                            ));
                            triples.push(Triple::new(
                                &subject,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_vg, "path",
                                ),
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_path, tokens[1],
                                ),
                            ));

                            //println!("{} {}", step, step.len());
                            node = &step[..(step.len() - 1)];
                            orientation = &step[(step.len() - 1)..];

                            node_term = "node";
                            node_orientation = "f";
                            node_orientation_long = "ForwardStrandPosition";

                            if orientation == "-" {
                                node_term = "reverseOfNode";
                                node_orientation = "r";
                                node_orientation_long = "ReverseStrandPosition";
                            } else if orientation != "+" {
                                return Err("the node orientation is not valid");
                            }

                            next_position_in_path = position_in_path + node_to_length.get(node).unwrap();

                            beg = tokens[1].to_owned() + node_orientation + position_in_path.to_string().as_str();
                            end = tokens[1].to_owned() + node_orientation + next_position_in_path.to_string().as_str();

                            //println!("\t\tvg:{} node:{} ;\n\t\tfaldo:begin <{}> ;\n\t\tfaldo:end <{}> . ", node_term, node, beg, end);
                            triples.push(Triple::new(
                                &subject,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_vg, node_term,
                                ),
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_node, node,
                                ),
                            ));

                            let beg_node = &graph.create_uri_node(&Uri::new(beg));
                            let end_node = &graph.create_uri_node(&Uri::new(end));
                            triples.push(Triple::new(
                                &subject,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "begin",
                                ),
                                &beg_node,
                            ));
                            triples.push(Triple::new(
                                &subject,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "end",
                                ),
                                &end_node,
                            ));

                            //println!("\t\t<{}> a faldo:ExactPosition,\n\t\t\tfaldo:{} ;\n\t\t\tfaldo:position {} ;\n\t\t\tfaldo:reference <{}> . ", beg, node_orientation_long, position_in_path, tokens[1]);
                            triples.push(Triple::new(
                                &beg_node,
                                &a,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "ExactPosition",
                                ),
                            ));
                            triples.push(Triple::new(
                                &beg_node,
                                &a,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, node_orientation_long,
                                ),
                            ));
                            triples.push(Triple::new(
                                &beg_node,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "position",
                                ),
                                &graph.create_literal_node_with_data_type(
                                    position_in_path.to_string(),
                                    &XmlDataTypes::Integer.to_uri()
                                )
                            ));
                            triples.push(Triple::new(
                                &beg_node,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "reference",
                                ),
                                &graph.create_uri_node(&Uri::new(
                                    tokens[1].to_string()
                                )),
                            ));

                            //println!("\t\t<{}> a faldo:ExactPosition,\n\t\t\tfaldo:{} ;\n\t\t\tfaldo:position {} ;\n\t\t\tfaldo:reference <{}> . ", end, node_orientation_long, next_position_in_path, tokens[1]);
                            triples.push(Triple::new(
                                &end_node,
                                &a,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "ExactPosition",
                                ),
                            ));
                            triples.push(Triple::new(
                                &end_node,
                                &a,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, node_orientation_long,
                                ),
                            ));
                            triples.push(Triple::new(
                                &end_node,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "position",
                                ),
                                &graph.create_literal_node_with_data_type(
                                    position_in_path.to_string(),
                                    &XmlDataTypes::Integer.to_uri()
                                )
                            ));
                            triples.push(Triple::new(
                                &end_node,
                                &graph.create_uri_node_from_namespace_and_id(
                                    prefix_faldo, "reference",
                                ),
                                &graph.create_uri_node(&Uri::new(
                                    tokens[1].to_string()
                                )),
                            ));

                            position_in_path = next_position_in_path;
                        }
                    }
                }
                "H" => (), // Header
                _ => {
                    return Err("the input file is not a valid GFA");
                }
            };

            println!("{}", writer.write_triples_on_the_fly( triples, false).unwrap());
        }
    }

    //println!("{}", writer.write_to_string(&graph).unwrap());

    return Ok("Done");
}