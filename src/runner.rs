use crate::graph::{Edge, Node};

struct GraphMachine {
    current_node: String,
    graph: HashMap<String, Node>,
}

trait Runner {
    fn choices(&self) -> Option<Vec<(Option<String>, &str)>>;
    fn traverse(&mut self, destination: String);
    fn get_node_label(&self) -> Option<&str>;
    fn get_command(&self) -> Option<&str>;
}

impl Runner for GraphMachine {
    fn choices(&self) -> Option<Vec<(Option<String>, &str)>> {
        let edges = match self.graph.get(&self.current_node) {
            Some(node) => &node.outputs,
            None => return None,
        };
        let choices = match edges.len() {
            0 => return None,
            _ => edges
                .iter()
                .map(|x| (x.label.clone(), x.destination.as_str()))
                .collect(),
        };
        Some(choices)
    }

    fn traverse(&mut self, destination: String) {
        self.current_node = destination;
    }

    fn get_node_label(&self) -> Option<&str> {
        match self.graph.get(&self.current_node) {
            Some(node) => match &node.label {
                Some(command) => Some(command.as_str()),
                None => None,
            },
            None => None,
        }
    }

    fn get_command(&self) -> Option<&str> {
        match self.graph.get(&self.current_node) {
            Some(node) => match &node.cmd {
                Some(command) => Some(command.as_str()),
                None => None,
            },
            None => None,
        }
    }
}