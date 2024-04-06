
#[derive(Debug)]
pub struct Node {
    pub outputs: Vec<Edge>,
    pub label: Option<String>,
    pub cmd: Option<String>,
}

#[derive(Debug)]
pub struct Edge {
    pub destination: String,
    pub label: Option<String>,
}
