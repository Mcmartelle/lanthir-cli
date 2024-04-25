use crate::graph::{Edge, Node};
use anyhow::Result;
use pest::Parser as PestParser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "mermaid.pest"]
pub struct MermaidParser;

pub fn parse_mermaid(flowchart_string: &str) -> Result<HashMap<String, Node>> {
    let mermaid_parts = MermaidParser::parse(Rule::mmd, flowchart_string)
        .expect("unsuccessful pest parse")
        .next()
        .unwrap();
    let mut graph: HashMap<String, Node> = HashMap::new();

    for part in mermaid_parts.into_inner() {
        match part.as_rule() {
            Rule::line => {
                let mut line_node_clusters: Vec<
                    Vec<(String, Option<String>, Option<String>, Option<String>)>,
                > = Vec::new();
                let mut line_edges: Vec<(bool, Option<String>)> = Vec::new();
                // let mut node_index: u8 = 0;
                // let mut edge_index: u8 = 0;
                // Line;
                for pair in part.into_inner() {
                    match pair.as_rule() {
                        Rule::node_cluster => {
                            // Node Cluster
                            let mut cluster_nodes: Vec<(
                                String,
                                Option<String>,
                                Option<String>,
                                Option<String>,
                            )> = Vec::new();
                            for node in pair.into_inner() {
                                match node.as_rule() {
                                    Rule::node => {
                                        // Node
                                        let mut node_id: String = String::new();
                                        let mut node_text: Option<String> = None;
                                        let mut node_cmd: Option<String> = None;
                                        let mut node_cb: Option<String> = None;
                                        for node_attr in node.into_inner() {
                                            match node_attr.as_rule() {
                                                Rule::node_id => {
                                                    // Node ID
                                                    node_id = String::from(node_attr.as_str());
                                                }
                                                Rule::node_shape => {
                                                    for node_content in node_attr.into_inner() {
                                                        match node_content.as_rule() {
                                                            Rule::node_text => {
                                                                // Node Text
                                                                node_text = Some(String::from(
                                                                    node_content.as_str(),
                                                                ));
                                                            }
                                                            Rule::shell_cmd => {
                                                                // Shell CMD
                                                                for shell_command in
                                                                    node_content.into_inner()
                                                                {
                                                                    match shell_command.as_rule() {
                                                                        Rule::shell_text => {
                                                                            let mut command_string =
                                                                                String::new();
                                                                            for slice in
                                                                                shell_command
                                                                                    .into_inner()
                                                                            {
                                                                                match slice.as_rule() {
                                                                                    Rule::non_double_quote => command_string.push_str(slice.as_str()),
                                                                                    Rule::double_quote => command_string.push('"'),
                                                                                    _ => unreachable!(),
                                                                                }
                                                                            }
                                                                            node_cmd = Some(
                                                                                command_string,
                                                                            );
                                                                        }
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            }
                                                            Rule::clip_board => {
                                                                // Shell CMD
                                                                for clipboard_content in
                                                                    node_content.into_inner()
                                                                {
                                                                    match clipboard_content
                                                                        .as_rule()
                                                                    {
                                                                        Rule::shell_text => {
                                                                            let mut
                                                                            clipboard_string =
                                                                                String::new();
                                                                            for slice in
                                                                                clipboard_content
                                                                                    .into_inner()
                                                                            {
                                                                                match slice.as_rule() {
                                                                                    Rule::non_double_quote => clipboard_string.push_str(slice.as_str()),
                                                                                    Rule::double_quote => clipboard_string.push('"'),
                                                                                    _ => unreachable!(),
                                                                                }
                                                                            }
                                                                            node_cb = Some(
                                                                                clipboard_string,
                                                                            );
                                                                        }
                                                                        _ => unreachable!(),
                                                                    }
                                                                }
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                        cluster_nodes.push((node_id, node_text, node_cmd, node_cb));
                                    }
                                    _ => {
                                        unreachable!()
                                    }
                                }
                            }
                            line_node_clusters.push(cluster_nodes);
                        }
                        Rule::edge => {
                            let mut line_edge: Option<String> = None;
                            let mut edge_is_directed = false;
                            for edge in pair.into_inner() {
                                match edge.as_rule() {
                                    Rule::directed_edge => {
                                        edge_is_directed = true;
                                        for edge_part in edge.into_inner() {
                                            match edge_part.as_rule() {
                                                Rule::edge_piped_text => {
                                                    for edge_text in edge_part.into_inner() {
                                                        match edge_text.as_rule() {
                                                            Rule::edge_text => {
                                                                line_edge = Some(String::from(
                                                                    edge_text.as_str(),
                                                                ));
                                                            }
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                }
                                                Rule::edge_text => {
                                                    line_edge =
                                                        Some(String::from(edge_part.as_str()));
                                                }
                                                _ => unreachable!(),
                                            }
                                        }
                                    }
                                    Rule::undirected_edge => {
                                        edge_is_directed = false;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            line_edges.push((edge_is_directed, line_edge));
                        }
                        _ => unreachable!(),
                    }
                }
                // Process line data into graph here
                // println!("line_node_clusters: {:#?}", line_node_clusters);
                // println!("line_edges: {:#?}", line_edges);
                assert_eq!(line_node_clusters.len(), line_edges.len() + 1);
                for index in 0..line_edges.len() {
                    if line_edges[index].0 {
                        for src_node in line_node_clusters[index].clone() {
                            for dest_node in line_node_clusters[index + 1].clone() {
                                graph
                                    .entry(src_node.0.clone())
                                    .and_modify(|entry| {
                                        modify_src_entry(
                                            entry,
                                            src_node.clone(),
                                            line_edges[index].clone(),
                                            dest_node.clone(),
                                        )
                                    })
                                    .or_insert(Node {
                                        outputs: vec![Edge {
                                            destination: dest_node.0.clone(),
                                            label: line_edges[index].1.clone(),
                                        }],
                                        label: src_node.1.clone(),
                                        cmd: src_node.2.clone(),
                                        cb: src_node.3.clone(),
                                    });
                                graph
                                    .entry(dest_node.0.clone())
                                    .and_modify(|entry| modify_dest_entry(entry, dest_node.clone()))
                                    .or_insert(Node {
                                        outputs: vec![],
                                        label: dest_node.1.clone(),
                                        cmd: dest_node.2.clone(),
                                        cb: dest_node.3.clone(),
                                    });
                            }
                        }
                    }
                }
            }
            Rule::header => {}
            _ => unreachable!(),
        }
    }

    Ok(graph)
}

fn modify_src_entry(
    entry: &mut Node,
    src_node: (String, Option<String>, Option<String>, Option<String>),
    edge: (bool, Option<String>),
    dest_node: (String, Option<String>, Option<String>, Option<String>),
) {
    entry.outputs.push(Edge {
        destination: dest_node.0.clone(),
        label: edge.1.clone(),
    });
    if src_node.1.is_some() {
        // overwriting previous values to match mermaid :rolling-eyes:
        entry.label = src_node.1;
    }
    if src_node.2.is_some() {
        entry.cmd = src_node.2;
    }
    if src_node.3.is_some() {
        entry.cb = src_node.3;
    }
}

fn modify_dest_entry(
    entry: &mut Node,
    dest_node: (String, Option<String>, Option<String>, Option<String>),
) {
    if dest_node.1.is_some() {
        entry.label = dest_node.1;
    }
    if dest_node.2.is_some() {
        entry.cmd = dest_node.2;
    }
    if dest_node.3.is_some() {
        entry.cb = dest_node.3;
    }
}
