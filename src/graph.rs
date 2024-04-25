use std::fmt;

#[derive(Debug)]
pub struct Node {
    pub outputs: Vec<Edge>,
    pub label: Option<String>,
    pub cmd: Option<String>,
    pub cb: Option<String>,
}

#[derive(Debug)]
pub struct Edge {
    pub destination: String,
    pub label: Option<String>,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.label {
            Some(x) => write!(f, "{} -> {}", x, &self.destination),
            None => write!(f, "to {}", &self.destination),
        }
    }
}
