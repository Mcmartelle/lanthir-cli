use anyhow::Result;
use pest::Parser as PestParser;
use std::fmt;

#[derive(pest_derive::Parser)]
#[grammar = "oats.pest"]
pub struct OatsParser;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Marker {
    Unordered,
    OneOf,
    AndThen,
    Clipbo,
    Breaker,
}

#[derive(Clone)]
pub struct Groat {
    pub marker: Option<Marker>,
    pub content: Option<String>,
}

impl fmt::Display for Groat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "marker: {:?}, content: {:?}",
            &self.marker, &self.content
        )
    }
}

pub fn parse_oats(oats_string: &str, verbose: bool) -> Result<Vec<Groat>> {
    let oats_parts = OatsParser::parse(Rule::oats, oats_string)
        .expect("unsuccessful pest parse")
        .next()
        .unwrap();
    let mut nodes: Vec<Groat> = Vec::new();

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
                        Rule::content => content = Some(String::from(pair.as_str().trim())),
                        Rule::breaker => marker = Some(Marker::Breaker),
                        _ => unreachable!(),
                    }
                }
                nodes.push(Groat {
                    marker: marker,
                    content: content,
                });
            }
            Rule::breaker => nodes.push(Groat {
                marker: Some(Marker::Breaker),
                content: None,
            }),
            _ => unreachable!(),
        }
    }
    // Process line data into graph here

    if verbose {
        for grain in &nodes {
            println!("{}", grain);
        }
    }

    return Ok(nodes);
}
