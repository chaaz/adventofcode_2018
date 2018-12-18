use std::thread::sleep;
use std::time::Duration;
use std::iter::{once, repeat};
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

pub fn run() {
  let mut content = include_bytes!("input.txt").to_vec();
  let mut map = Map::from_bytes(&mut content);

  let rounds = repeat(|| ())
    .enumerate()
    .map(|_| { sleep(Duration::new(0, 500_000_000)); map.print(); map.round() })
    .take_while(|b| !b)
    .count();
  let hp = map.total_hp() as usize;

  println!("rounds {} x hp_total {} = {}\n", rounds, hp, rounds * hp);
  println!("{}", if map.winner() == b'E' { "Elves" } else { "Goblins" });
  map.print();
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
  pub x: usize,
  pub y: usize
}

impl Point {
  pub fn new(x: usize, y: usize) -> Point { Point { x, y } }

  pub fn is_adj(&self, other: &Point) -> bool {
    (self.x == other.x &&
      (self.y == other.y + 1 || self.y + 1 == other.y)) ||
      (self.y == other.y &&
      (self.x == other.x + 1 || self.x + 1 == other.x))
  }

  pub fn adj(&self) -> Vec<Point> {
    let mut vec = Vec::new();
    if self.y > 0 { vec.push(Point::new(self.x, self.y - 1)); }
    if self.x > 0 { vec.push(Point::new(self.x - 1, self.y)); }
    vec.push(Point::new(self.x + 1, self.y));
    vec.push(Point::new(self.x, self.y + 1));
    vec
  }

  fn dist(&self, other: &Point) -> usize {
    let mut sum = 0;

    if self.x > other.x { sum += self.x - other.x; }
    else { sum += other.x - self.x; }

    if self.y > other.y { sum += self.y - other.y; }
    else { sum += other.y - self.y; }

    sum
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Point {
  fn cmp(&self, other: &Point) -> Ordering {
    let Point { x: x0, y: y0 } = self;
    let Point { x: x1, y: y1 } = other;

    if y0 < y1 { Ordering::Less }
    else if y0 > y1 { Ordering::Greater }
    else if x0 < x1 { Ordering::Less }
    else if x0 > x1 { Ordering::Greater }
    else { Ordering::Equal }
  }
}

type Rows<'a> = Vec<&'a mut [u8]>;

struct Map<'a> {
  rows: Rows<'a>,
  units: Vec<(Point, Unit)>
}

impl<'a> Map<'a> {
  fn new(rows: Rows<'a>, units: Vec<(Point, Unit)>) -> Map {
    Map { rows, units }
  }

  fn from_bytes(content: &mut Vec<u8>) -> Map {
    let mut rows: Rows = content.split_mut(|b| *b == b'\n').collect();

    let mut units = Vec::new();
    EnumChars::new(&rows)
      .filter(|(_, pt)| pt == &b'E' || pt == &b'G')
      .for_each(|(pt, u)| units.push((pt, Unit::init(u))));
    for (pt, _) in &units { rows[pt.y][pt.x] = b'.'; }

    Map::new(rows, units)
  }

  fn enum_chars<'b>(&'b self) -> EnumChars<'b, 'a> {
    EnumChars::new(&self.rows)
  }

  fn print(&self) {
    let mut rows: Vec<_> = self.rows.iter().map(|r| r.to_vec()).collect();
    for (pt, u) in &self.units { rows[pt.y][pt.x] = u.team; }

    for (y, row) in rows.iter().enumerate() {
      print!("{}", std::str::from_utf8(row).unwrap());
      self.print_hps(row, y);
      println!("");
    }
  }

  fn print_hps(&self, row: &[u8], y: usize) {
    for (x, ch) in row.iter().enumerate() {
      match ch {
        b'E' => print!(" E({})", self.find_ok(&Point::new(x, y)).hp),
        b'G' => print!(" G({})", self.find_ok(&Point::new(x, y)).hp),
        _ => ()
      }
    }
  }

  fn find(&self, pt: &Point) -> Option<&Unit> {
    self.units.iter().find(|(upt, _)| upt == pt).map(|(_, u)| u)
  }

  fn find_ok(&self, pt: &Point) -> &Unit { self.find(pt).unwrap() }

  fn find_mut(&mut self, pt: &Point) -> &mut Unit {
    self.units.iter_mut().find(|(upt, _)| upt == pt).map(|(_, u)| u).unwrap()
  }

  fn remove(&mut self, pt: &Point) {
    self.units.retain(|(upt, _)| upt != pt);
  }

  fn round(&mut self) -> bool {
    self.units.sort_by_key(|(pt, _)| pt.clone());
    let pts = self.units.iter().map(|(pt, _)| pt).cloned().collect::<Vec<_>>();
    for pt in pts {
      if self.turn(&pt) { return true; }
    }
    false
  }

  fn turn(&mut self, pt: &Point) -> bool {
    let enemy_team = match self.find(pt) {
      Some(unit) => unit.enemy_team(),
      None => return false    // we've been killed
    };

    if !self.any_enemies(enemy_team) {
      true
    } else {
      self.turn_action(pt, enemy_team);
      false
    }
  }

  fn turn_action(&mut self, pt: &Point, enemy_team: u8) {
    if let Some(loc) = self.find_best_adj_enemy(pt, enemy_team) {
      self.attack(pt, &loc);
    } else {
      if let Some(step) = self.find_best_step(pt, enemy_team) {
        let pt = self.take_step(pt, step);
        if let Some(loc) = self.find_best_adj_enemy(&pt, enemy_team) {
          self.attack(&pt, &loc);
        }
      }
    }
  }

  fn winner(&self) -> u8 { self.units[0].1.team }

  fn any_enemies(&self, enemy: u8) -> bool {
    self.units.iter().any(|(_, u)| u.team == enemy)
  }

  fn find_best_step(&self, pt: &Point, enemy_team: u8) -> Option<Point> {
    let ranges = self.ranges(enemy_team);
    let best = self.a_star(pt, &ranges);
    best.and_then(|p| p.get(1).cloned())
  }

  fn take_step(&mut self, pt: &Point, step: Point) -> Point {
    self.units.iter_mut().find(|(upt, _)| upt == pt).unwrap().0 = step.clone();
    step
  }

  fn attack(&mut self, attacker: &Point, target: &Point) {
    let power = self.find_ok(attacker).power;
    let hp = self.find_ok(target).hp;

    if hp > power {
      self.find_mut(target).hp -= power;
    } else {
      self.remove(target);
    }
  }

  fn ranges(&self, enemy_team: u8) -> HashSet<Point> {
    self.units.iter().filter(|(_, u)| u.team == enemy_team)
      .flat_map(|(pt, _)| pt.adj().into_iter())
      .filter(|pt| self.is_unoccupied(pt))
      .collect()
  }

  fn is_unoccupied(&self, pt: &Point) -> bool {
      self.rows[pt.y][pt.x] == b'.'
        && !self.units.iter().any(|(upt, _)| upt == pt)
  }

  fn find_best_adj_enemy(&self, pt: &Point, enemy_team: u8) -> Option<Point> {
    self.units.iter()
      .filter(|(upt, u)| u.team == enemy_team && upt.is_adj(pt))
      .min_by(|(upt0, u0), (upt1, u1)| {
        match u0.hp.cmp(&u1.hp) {
          Ordering::Equal => upt0.cmp(upt1),
          other => other
        }
      })
      .map(|(upt, _)| upt.clone())
  }

  fn a_star(&self, pt: &Point, ranges: &HashSet<Point>) -> Option<Vec<Point>> {
    if ranges.is_empty() { return None; }

    let mut nodes = self.nodes(pt, ranges);
    let mut open: HashSet<_> = once(pt.clone()).collect();
    let mut closed = HashSet::new();

    while !open.is_empty() {
      let current = open
        .iter()
        .min_by(|l0, l1| nearest(*l0, *l1, &nodes))
        .unwrap()
        .clone();

      let cur_g = nodes[&current].g;
      if ranges.contains(&current) {
        return Some(reconstruct(&current, &nodes));
      }

      open.remove(&current);
      closed.insert(current.clone());

      for neighbor in neighbors(&current, &nodes) {
        let neighbor_node = nodes.get_mut(&neighbor).unwrap();
        if closed.contains(&neighbor) { continue; }
        let neighbor_g = cur_g + 1;
        open.insert(neighbor.clone());
        if neighbor_g > neighbor_node.g { continue; }
        if neighbor_g == neighbor_node.g
            && neighbor_node.prev.as_ref().unwrap() < &current {
          continue;
        }

        neighbor_node.prev = Some(current.clone());
        neighbor_node.g = neighbor_g;
        neighbor_node.f = neighbor_g +
          ranges.iter().map(|r| r.dist(&neighbor)).min().unwrap();
      }
    }

    None
  }

  fn nodes(&self, pt: &Point, ranges: &HashSet<Point>) -> Nodes {
    self.enum_chars()
      .filter(|(upt, _)| self.is_unoccupied(&upt) || upt == pt)
      .map(|(upt, _)| (
        upt.clone(),
        Node {
          loc: upt.clone(),
          prev: None,
          g: if &upt == pt { 0 } else { std::usize::MAX },
          f: if &upt == pt {
            ranges.iter().map(|r| r.dist(pt)).min().unwrap()
          } else {
            std::usize::MAX
          }
        }
      ))
      .collect()
  }

  fn total_hp(&self) -> u32 { self.units.iter().map(|v| v.1.hp).sum() }
}

struct EnumChars<'a, 'm> {
  rows: &'a Rows<'m>,
  row: usize,
  col: usize
}

impl<'a, 'm> EnumChars<'a, 'm> {
  pub fn new(rows: &'a Rows<'m>) -> EnumChars<'a, 'm> {
    EnumChars { rows, row: 0, col: 0 }
  }
}

impl<'a, 'm> Iterator for EnumChars<'a, 'm> {
  type Item = (Point, u8);

  fn next(&mut self) -> Option<(Point, u8)> {
    if self.row >= self.rows.len() {
      return None;
    }

    let r = (Point::new(self.col, self.row), self.rows[self.row][self.col]);

    self.col += 1;
    while self.row < self.rows.len() && self.col >= self.rows[self.row].len() {
      self.col = 0;
      self.row += 1;
    }

    Some(r)
  }
}

struct Unit {
  pub team: u8,
  pub power: u32,
  pub hp: u32
}

impl Unit {
  pub fn init(team: u8) -> Unit {
    let power = if team == b'G' { 3 } else { 18 };
    Unit { team, power, hp: 200 }
  }

  pub fn enemy_team(&self) -> u8 {
    match self.team {
      b'E' => b'G',
      b'G' => b'E',
      _ => panic!("Unknown team: {}", self.team)
    }
  }
}

type Nodes = HashMap<Point, Node>;

struct Node {
  pub loc: Point,
  pub prev: Option<Point>,
  pub g: usize,
  pub f: usize,
}

fn nearest(c0: &Point, c1: &Point, nodes: &Nodes) -> Ordering {
  nearest_pt_val(c0, nodes[c0].f, c1, nodes[c1].f)
}

fn nearest_pt_val(c0: &Point, min0: usize,
                  c1: &Point, min1: usize) -> Ordering {
  match min0.cmp(&min1) {
    Ordering::Equal => c0.cmp(c1),
    other => other
  }
}

fn reconstruct(loc: &Point, nodes: &Nodes) -> Vec<Point> {
  let mut vec = Vec::new();
  let mut loc = loc.clone();
  vec.push(loc.clone());
  while let Some(prev) = &nodes[&loc].prev {
    vec.push(prev.clone());
    loc = prev.clone();
  }

  vec.into_iter().rev().collect()
}

fn neighbors(cur: &Point, nodes: &Nodes) -> Vec<Point> {
  cur.adj().into_iter().filter(|a| nodes.contains_key(&a)).collect()
}
