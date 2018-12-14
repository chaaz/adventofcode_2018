use std::collections::HashMap;

pub fn run() {
  let content = include_str!("input.txt").trim().split("\n");

  let mut graph = Graph::new();
  for req in content.map(Req::from_line) { graph.add_req(req); }

  let mut available = graph.init();
  while !available.is_empty() {
    available.sort();
    let next = available.remove(0);
    print!("{}", next);
    graph.process(&next, &mut available);
  }
  print!("\n");
}

struct Graph {
  nodes: HashMap<String, Node>
}

impl Graph {
  pub fn new() -> Graph { Graph { nodes: HashMap::new() } }

  pub fn init(&self) -> Vec<String> {
    let mut v = Vec::new();
    for (_, n) in &self.nodes {
      if n.reqs.is_empty() {
        v.push(n.name.clone());
      }
    }
    v
  }

  pub fn process(&mut self, name: &str, avail: &mut Vec<String>) {
    for (_, node) in &mut self.nodes {
      node.reqs.retain(|s| s != name);
    }

    let did = self.nodes.remove(name).unwrap();
    for dep in &did.deps {
      let n = self.nodes.get(dep).unwrap();
      if n.reqs.is_empty() {
        avail.push(n.name.clone());
      }
    }
  }

  pub fn add_req(&mut self, req: Req) {
    let f = req.first;
    let then = req.then;

    let n0 = self.nodes.entry(f.clone()).or_insert(Node::new(f.clone()));
    n0.add_dep(&then);

    let n1 = self.nodes.entry(then.clone()).or_insert(Node::new(then));
    n1.add_req(&f);
  }
}

struct Node {
  name: String,
  reqs: Vec<String>,
  deps: Vec<String>
}

impl Node {
  pub fn new(name: String) -> Node {
    Node { name, reqs: Vec::new(), deps: Vec::new() }
  }

  pub fn add_req(&mut self, name: &str) {
    if !self.reqs.iter().any(|r| r == name) {
      self.reqs.push(name.to_string());
    }
  }

  pub fn add_dep(&mut self, name: &str) {
    if !self.deps.iter().any(|r| r == name) {
      self.deps.push(name.to_string());
    }
  }
}

struct Req {
  first: String,
  then: String
}

impl Req {
  pub fn from_line(line: &str) -> Req {
    let first = &line.split(" must be finished").next().unwrap()[5 ..];
    let then = &line.split("before step ").nth(1).unwrap();
    let then = then.split(" ").next().unwrap();

    Req { first: first.to_string(), then: then.to_string() }
  }
}
