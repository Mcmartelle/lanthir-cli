use anyhow::Result;
use pest::Parser as PestParser;
use std::fmt;

#[derive(pest_derive::Parser)]
#[grammar = "checklist.pest"]
pub struct ChecklistParser;

#[derive(Clone)]
pub struct Vertex {
    pub alone: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub wrapper: Option<String>,
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "alone: {:?}, before: {:?}, after: {:?}, wrapper: {:?}",
            &self.alone, &self.before, &self.after, &self.wrapper
        )
    }
}

pub fn parse_checklist(
    checklist_string: &str,
    verbose: bool,
    unordered: bool,
) -> Result<Vec<Vertex>> {
    let checklist_parts = ChecklistParser::parse(Rule::checklist, checklist_string)
        .expect("unsuccessful pest parse")
        .next()
        .unwrap();
    let mut nodes: Vec<Vertex> = Vec::new();

    for part in checklist_parts.into_inner() {
        match part.as_rule() {
            Rule::line => {
                // let mut edge_index: u8 = 0;
                // Line;
                let mut node_alone: Option<String> = None;
                let mut node_before: Option<String> = None;
                let mut node_after: Option<String> = None;
                let mut node_wrapper: Option<String> = None;
                for pair in part.into_inner() {
                    match pair.as_rule() {
                        Rule::alone => node_alone = Some(String::from(pair.as_str())),
                        Rule::before => node_before = Some(String::from(pair.as_str())),
                        Rule::after => node_after = Some(String::from(pair.as_str())),
                        Rule::wrapper => {
                            for node_content in pair.into_inner() {
                                match node_content.as_rule() {
                                    Rule::inner => {
                                        node_wrapper = Some(String::from(node_content.as_str()))
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                nodes.push(Vertex {
                    alone: node_alone,
                    before: node_before,
                    after: node_after,
                    wrapper: node_wrapper,
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
    if unordered {
        println!("unordered currently not supported");
    }

    return Ok(nodes);
}
