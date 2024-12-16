use anyhow::Result;
use pest::Parser as PestParser;
use std::fmt;

#[derive(pest_derive::Parser)]
#[grammar = "oats.pest"]
pub struct OatsParser;

#[derive(Debug, Clone)]
enum Marker {
    Unordered,
    OneOf,
    AndThen,
    Clipbo,
    Breaker,
}

#[derive(Clone)]
pub struct Vertex {
    pub marker: Option<Marker>,
    pub content: Option<String>,
    pub done: bool,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "marker: {:?}, content: {:?}, done: {:?}",
            &self.marker, &self.content, &self.done
        )
    }
}

pub fn parse_oats(oats_string: &str, verbose: bool, unordered: bool) -> Result<Vec<Vertex>> {
    let oats_parts = OatsParser::parse(Rule::oats, oats_string)
        .expect("unsuccessful pest parse")
        .next()
        .unwrap();
    let mut nodes: Vec<Vertex> = Vec::new();

    for part in oats_parts.into_inner() {
        match part.as_rule() {
            Rule::node => {
                let mut content: Option<String> = None;
                let mut marker: Option<Marker> = None;
                for pair in part.into_inner() {
                    match pair.as_rule() {
                        Rule::marker => {
                            for node_marker in pair.into_inner() {
                                match node_marker.as_rule() {
                                    Rule::unordered => marker = Some(Marker::Unordered),
                                    Rule::one_of => marker = Some(Marker::OneOf),
                                    Rule::and_then => marker = Some(Marker::AndThen),
                                    Rule::clipbo => marker = Some(Marker::Clipbo),
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Rule::content => content = Some(String::from(pair.as_str())),
                        Rule::breaker => marker = Some(Marker::Breaker),
                        _ => unreachable!(),
                    }
                }
                nodes.push(Vertex {
                    marker: marker,
                    content: content,
                    done: false,
                });
            }
            _ => unreachable!(),
        }
    }
    // Process line data into graph here

    if verbose {
        for vector in &nodes {
            println!("{}", vector);
        }
    }

    return Ok(nodes);
}
