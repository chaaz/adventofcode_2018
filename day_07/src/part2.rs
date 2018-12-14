use std::collections::HashMap;

const OVERHEAD: u32 = 60;
const POOL_SIZE: usize = 5;

pub fn run() {
  let mut graph = Graph::new();
  let content = include_str!("input.txt").trim().split("\n");
  for req in content.map(Req::from_line) { graph.add_req(req); }

  let mut workers = WorkPool::new(POOL_SIZE);
  let mut available = graph.init();
  while !available.is_empty() || workers.has_active() {
    available.sort();
    while !available.is_empty() && workers.has_idle() {
      let next = available.remove(0);
      workers.assign(next.clone());
    }
    for task in workers.tick_until_done() {
      graph.process(&task, &mut available);
    }
  }
  println!("\nDone in {}", workers.clock);
}

struct WorkPool {
  clock: u32,
  workers: Vec<Worker>,
}

impl WorkPool {
  pub fn new(size: usize) -> WorkPool {
    WorkPool { clock: 0, workers: vec![Worker::new(); size] }
  }

  pub fn has_idle(&self) -> bool { self.workers.iter().any(|w| w.is_idle()) }
  pub fn has_active(&self) -> bool { self.workers.iter().any(|w| !w.is_idle()) }

  pub fn assign(&mut self, task: String) {
    assert!(task.len() == 1);
    let idle = self.workers.iter_mut().find(|w| w.is_idle()).unwrap();
    let time = OVERHEAD + ((*&task.as_bytes()[0]) - b'A' + 1) as u32;
    idle.assign(task, time);
  }

  pub fn tick_until_done(&mut self) -> Vec<String> {
    let t = self.workers.iter().min_by_key(|w| {
      if w.remaining > 0 { w.remaining } else { std::u32::MAX }
    }).unwrap().remaining;
    assert!(t <= 90, "Too long / no task.");
    let mut done = Vec::new();
    for w in &mut self.workers { if let Some(v) = w.tick(t) { done.push(v);} }
    assert!(!done.is_empty(), "No completes after min tick");
    self.clock += t;
    done.sort();
    done
  }
}

#[derive(Clone)]
struct Worker {
  task: Option<String>,
  remaining: u32
}

impl Worker {
  pub fn new() -> Worker { Worker { task: None, remaining: 0 } }

  pub fn assign(&mut self, task: String, time: u32) {
    self.task = Some(task);
    self.remaining = time;
  }

  pub fn is_idle(&self) -> bool { self.task.is_none() }

  pub fn tick(&mut self, t: u32) -> Option<String> {
    if self.task.is_some() {
      self.remaining -= t;
      if self.remaining == 0 {
        self.task.take()
      } else {
        None
      }
    } else {
      None
    }
  }
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
    let first = req.first;
    let first2 = first.clone();
    let first3 = first.clone();
    let then = req.then;
    let then2 = then.clone();

    let n0 = self.nodes.entry(first2).or_insert(Node::new(first3));
    n0.add_dep(&then);

    let n1 = self.nodes.entry(then2).or_insert(Node::new(then));
    n1.add_req(&first);
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
