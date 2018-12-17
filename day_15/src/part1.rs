use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

pub fn run() {
  let mut content: Vec<u8> = Vec::new();
  content.extend_from_slice(include_bytes!("input.txt"));
  let mut map = Map::from_bytes(&mut content[..]);

  let mut rounds = 0;
  loop {
    println!("round {}:", rounds);
    if map.round() { break; }
    rounds += 1;
  }

  let hp = map.total_hp();
  println!("rounds {} x hp_total {} = {}\n", rounds, hp, rounds * hp);
  println!("winner: {}",
           if map.winner() == b'E' { "Elves" } else { "Goblins" });
  map.print();
}

struct Map<'a> {
  rows: Vec<&'a mut [u8]>,
  units: HashMap<(usize, usize), Unit>
}

impl<'a> Map<'a> {
  fn new(rows: Vec<&'a mut [u8]>, units: HashMap<(usize, usize), Unit>) -> Map {
    Map { rows, units }
  }

  fn print(&self) {
    for (y, row) in self.rows.iter().enumerate() {
      print!("{}", std::str::from_utf8(row).unwrap());
      for (x, unit) in row.iter().enumerate() {
        match unit {
          b'E' => print!(" E({})", self.units[&(x, y)].hp),
          b'G' => print!(" G({})", self.units[&(x, y)].hp),
          _ => ()
        }
      }
      println!("");
    }
  }

  fn from_bytes(content: &mut [u8]) -> Map {
    let rows: Vec<_> = content
      .split_mut(|b| *b == b'\n')
      .collect();

    let mut units = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
      for (x, unit) in row.iter().enumerate() {
        match unit {
          b'E' | b'G' => { units.insert((x, y), Unit::init()); }
          _ => ()
        }
      }
    }

    Map::new(rows, units)
  }

  fn round(&mut self) -> bool {
    let mut hits = Vec::new();

    for y in 0 .. self.rows.len() {
      for x in 0 .. self.rows[y].len() {
        let unit = self.rows[y][x];
        if unit == b'E' || unit == b'G' {
          hits.push((x, y));
        }
      }
    }

    for (x, y) in hits {
      let unit = self.rows[y][x];
      if unit == b'E' || unit == b'G' {
        if self.turn(x, y, unit) { return true; }
      }
    }

    false
  }

  fn turn(&mut self, x: usize, y: usize, unit: u8) -> bool {
    if !self.any_enemies(unit) { return true; }

    if let Some(loc) = self.find_best_adj_target(x, y, unit) {
      self.attack((x, y), loc);
    } else {
      if let Some(step) = self.find_best_step(x, y, unit) {
        let (x, y) = self.take_step(x, y, step);
        if let Some(loc) = self.find_best_adj_target(x, y, unit) {
          self.attack((x, y), loc);
        }
      }
    }

    false
  }

  fn winner(&self) -> u8 {
    for y in 0 .. self.rows.len() {
      for x in 0 .. self.rows[y].len() {
        let unit = self.rows[y][x];
        if unit == b'G' || unit == b'E' {
          return unit;
        }
      }
    }
    unreachable!()
  }

  fn any_enemies(&self, unit: u8) -> bool {
    let enemy = Map::enemy(unit);
    for y in 0 .. self.rows.len() {
      for x in 0 .. self.rows[y].len() {
        let unit = self.rows[y][x];
        if unit == enemy {
          return true;
        }
      }
    }
    false
  }

  fn find_best_step(&self, x: usize, y: usize, unit: u8)
  -> Option<(usize, usize)> {
    let ranges = self.ranges(unit);
    let best = self.best_path(x, y, &ranges);
    best.and_then(|p| p.get(1).cloned())
  }

  fn take_step(&mut self, x: usize, y: usize, step: (usize, usize))
  -> (usize, usize) {
    self.rows[step.1][step.0] = self.rows[y][x];
    self.rows[y][x] = b'.';

    let unit = self.units.remove(&(x, y)).unwrap();
    self.units.insert(step, unit);

    step
  }

  fn find_best_adj_target(&self, x: usize, y: usize, unit: u8)
  -> Option<(usize, usize)> {
    let enemy = Map::enemy(unit);
    let mut ranges = Vec::new();
    self.push_adj_eq(x, y, &mut ranges, enemy);

    ranges.sort_by(|l0, l1| {
      let hp0 = self.units[l0].hp;
      let hp1 = self.units[l1].hp;
      if hp0 < hp1 { return Ordering::Less; }
      if hp0 > hp1 { return Ordering::Greater; }
      nearest_pt(*l0, *l1)
    });

    ranges.first().cloned()
  }

  fn attack(&mut self, attacker: (usize, usize), target: (usize, usize)) {
    if self.units[&target].hp > self.units[&attacker].power {
      self.units.get_mut(&target).unwrap().hp -= self.units[&attacker].power;
    } else {
      self.units.remove(&target);
      self.rows[target.1][target.0] = b'.';
    }
  }

  fn ranges(&self, unit: u8) -> HashSet<(usize, usize)> {
    let enemy = Map::enemy(unit);
    let mut ranges = HashSet::new();
    for y in 0 .. self.rows.len() {
      for x in 0 .. self.rows[y].len() {
        let unit = self.rows[y][x];
        if unit == enemy {
          self.add_adj_ranges(x, y, &mut ranges);
        }
      }
    }
    ranges
  }

  fn add_adj_ranges(&self, x: usize, y: usize,
                    ranges: &mut HashSet<(usize, usize)>) {
    let spot = b'.';
    self.insert_adj_eq(x, y, ranges, spot);
  }

  fn insert_adj_eq(&self, x: usize, y: usize,
                   ranges: &mut HashSet<(usize, usize)>, spot: u8) {
    if y > 0 && self.rows[y - 1][x] == spot { ranges.insert((x, y - 1)); }
    if x > 0 && self.rows[y][x - 1] == spot { ranges.insert((x - 1, y)); }
    if x < self.rows[y].len() - 1 &&
      self.rows[y][x + 1] == spot { ranges.insert((x + 1, y)); }
    if y < self.rows.len() - 1 &&
      self.rows[y + 1][x] == spot { ranges.insert((x, y + 1)); }
  }

  fn push_adj_eq(&self, x: usize, y: usize,
                 ranges: &mut Vec<(usize, usize)>, spot: u8) {
    if y > 0 && self.rows[y - 1][x] == spot { ranges.push((x, y - 1)); }
    if x > 0 && self.rows[y][x - 1] == spot { ranges.push((x - 1, y)); }
    if x < self.rows[y].len() - 1 &&
      self.rows[y][x + 1] == spot { ranges.push((x + 1, y)); }
    if y < self.rows.len() - 1 &&
      self.rows[y + 1][x] == spot { ranges.push((x, y + 1)); }
  }

  fn best_path(&self, x: usize, y: usize, ranges: &HashSet<(usize, usize)>)
  -> Option<Vec<(usize, usize)>> {
    let mut nodes = self.make_nodes(x, y, ranges);
    let mut open: HashMap<(usize, usize), HashSet<(usize, usize)>> =
      ranges.iter().map(|r| (*r, HashSet::new())).collect();
    for r in ranges { open.get_mut(r).unwrap().insert((x, y)); }
    let mut closed: HashMap<(usize, usize), HashSet<(usize, usize)>> =
      ranges.iter().map(|r| (*r, HashSet::new())).collect();

    while open.iter().any(|(_, o)| !o.is_empty()) {
      let fold: Vec<((usize, usize), (usize, usize), usize)> =
        open
          .iter()
          .filter_map(|(r, s)| {
            s.iter().min_by({
              |l0, l1| nearest(**l0, **l1, &nodes, *r)
            }).map(|b| (r, b))
          })
          .map(|(r, b)| (*r, *b, nodes[b.1][b.0].as_ref().unwrap().f[r]))
          .collect();

      let (r, current, _) = fold.iter().min_by(|(r0, b0, v0), (r1, b1, v1)| {
        nearest_pt_val_rng(*r0, *b0, *v0, *r1, *b1, *v1)
      }).unwrap();

      let cur_node = nodes[current.1][current.0].as_ref().unwrap();
      let cur_g = cur_node.g;
      if ranges.contains(current) {
        return Some(Map::reconstruct(current, &nodes));
      }

      open.get_mut(r).unwrap().remove(current);
      closed.get_mut(r).unwrap().insert(*current);

      for next in Map::neighbors(*current, &nodes) {
        let next_node = nodes[next.1][next.0].as_mut().unwrap();
        if closed[r].contains(&next) { continue; }
        let next_g = cur_g + 1;
        if !open[r].contains(&next) {
          open.get_mut(r).unwrap().insert(next);
        }
        if next_g > next_node.g { continue; }
        if next_g == next_node.g
            && read_order(next_node.prev.as_ref().unwrap(), current) {
          next_node.f.insert(*r, next_g + hdist(*r, next));
          continue;
        }

        next_node.prev = Some(current.clone());
        next_node.g = next_g;
        next_node.f.insert(*r, next_g + hdist(*r, next));
      }
    }

    None
  }

  fn make_nodes(&self, x: usize, y: usize, ranges: &HashSet<(usize, usize)>)
  -> Vec<Vec<Option<Node>>> {
    self.rows
      .iter()
      .enumerate()
      .map(|(ny, r)| r.iter().enumerate().map(|(nx, ch)| {
        if *ch == b'.' || (nx, ny) == (x, y) {
          Some(Node {
            loc: (nx, ny),
            prev: None,
            g: if (nx, ny) == (x, y) { 0 } else { std::usize::MAX },
            f: if (nx, ny) == (x, y) {
              ranges.iter().map(|k| (*k, hdist(*k, (x, y)))).collect()
            } else {
              ranges.iter().map(|k| (*k, std::usize::MAX)).collect()
            }
          })
        } else {
          None
        }
      }).collect()).collect()
  }

  fn neighbors(cur: (usize, usize), nodes: &Vec<Vec<Option<Node>>>)
  -> Vec<(usize, usize)> {
    let y = cur.1;
    let x = cur.0;
    let mut vec = Vec::new();
    if y > 0 &&
        nodes.get(y - 1).and_then(|r| r.get(x).and_then(|n| n.as_ref()))
          .is_some() {
      vec.push((x, y - 1));
    }
    if x > 0 &&
        nodes.get(y).and_then(|r| r.get(x - 1).and_then(|n| n.as_ref()))
          .is_some() {
      vec.push((x - 1, y));
    }
    if nodes.get(y + 1).and_then(|r| r.get(x).and_then(|n| n.as_ref()))
        .is_some() {
      vec.push((x, y + 1));
    }
    if nodes.get(y).and_then(|r| r.get(x + 1).and_then(|n| n.as_ref()))
        .is_some() {
      vec.push((x + 1, y));
    }

    vec
  }

  fn reconstruct(loc: &(usize, usize),
                 nodes: &Vec<Vec<Option<Node>>>) -> Vec<(usize, usize)> {
    let mut vec = Vec::new();
    vec.push(loc.clone());
    let mut loc = loc.clone();
    while let Some(prev) = nodes[loc.1][loc.0].as_ref().unwrap().prev {
      vec.push(prev.clone());
      loc = prev.clone();
    }

    vec.into_iter().rev().collect()
  }

  fn total_hp(&self) -> u32 {
    self.units.values().map(|u| u.hp).sum()
  }

  fn enemy(unit: u8) -> u8 {
    match unit {
      b'E' => b'G',
      b'G' => b'E',
      other => panic!("Not a unit: {}", other)
    }
  }
}

struct Unit {
  pub power: u32,
  pub hp: u32
}

impl Unit {
  pub fn init() -> Unit { Unit { power: 3, hp: 200 } }
}

struct Node {
  pub loc: (usize, usize),
  pub prev: Option<(usize, usize)>,
  pub g: usize,
  pub f: HashMap<(usize, usize), usize>
}

fn nearest(c0: (usize, usize), c1: (usize, usize),
           nodes: &Vec<Vec<Option<Node>>>,
           r: (usize, usize)) -> Ordering {
  let (x0, y0) = c0;
  let (x1, y1) = c1;

  let min0 = nodes[y0][x0].as_ref().unwrap().f[&r];
  let min1 = nodes[y1][x1].as_ref().unwrap().f[&r];
  nearest_pt_val(c0, min0, c1, min1)
}

fn nearest_pt_val_rng(r0: (usize, usize), c0: (usize, usize), min0: usize,
                      r1: (usize, usize), c1: (usize, usize), min1: usize)
-> Ordering {
  match nearest_pt_val(c0, min0, c1, min1) {
    Ordering::Equal => nearest_pt(r0, r1),
    other => other
  }
}

fn nearest_pt_val(c0: (usize, usize), min0: usize,
                  c1: (usize, usize), min1: usize) -> Ordering {
  if min0 < min1 { Ordering::Less }
  else if min0 > min1 { Ordering::Greater }
  else { nearest_pt(c0, c1) }
}

fn nearest_pt(c0: (usize, usize), c1: (usize, usize)) -> Ordering {
  let (x0, y0) = c0;
  let (x1, y1) = c1;

  if y0 < y1 { Ordering::Less }
  else if y0 > y1 { Ordering::Greater }
  else if x0 < x1 { Ordering::Less }
  else if x0 > x1 { Ordering::Greater }
  else { Ordering::Equal }
}

fn hdist(p0: (usize, usize), p1: (usize, usize)) -> usize {
  let (x0, y0) = p0;
  let (x1, y1) = p1;

  let mut sum = 0;
  if x0 > x1 { sum += x0 - x1; } else { sum += x1 - x0; }
  if y0 > y1 { sum += y0 - y1; } else { sum += y1 - y0; }
  sum
}

fn read_order(p0: &(usize, usize), p1: &(usize, usize)) -> bool {
  let (x0, y0) = p0;
  let (x1, y1) = p1;

  if y0 < y1 { return true };
  if y0 > y1 { return false };

  if x0 < x1 { return true };
  if x0 > x1 { return false };

  true
}
