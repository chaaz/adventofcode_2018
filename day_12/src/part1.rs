use std::fmt::{self, Debug};

pub fn run() {
  let mut content = include_str!("input.txt").trim().split("\n");

  let mut pots = PotLine::from_line(content.next().unwrap());
  let blank = content.next().unwrap().trim();
  assert!(blank.is_empty());
  let notes = Notes::from_iter(content.map(Pattern::from_line));

  pots.tick(20, &notes);
  println!("Total count: {}", pots.sum_plants());
}

#[derive(Clone)]
pub struct PotLine {
  segment_length: usize,
  segments: Vec<Vec<bool>>,
  left_ind: i64,
  left_plant: i64,
  right_plant: i64
}

impl PotLine {
  pub fn from_line(line: &str) -> PotLine {
    let line = line.split(": ").nth(1).unwrap().trim();
    let line = line.split("").filter(|c| !c.is_empty());
    let seg: Vec<_> = line.map(|c| c == "#").collect();

    let left = seg.iter().position(|p| *p).unwrap() as i64;
    let right = 
        (seg.len() - 1 - seg.iter().rev().position(|p| *p).unwrap()) as i64;
    PotLine {
      segment_length: seg.len(),
      segments: vec![seg],
      left_ind: 0,
      left_plant: left,
      right_plant: right
    }
  }

  pub fn tick(&mut self, count: usize, notes: &Notes) {
    for t in 0 .. count {
      self.println(t);
      self.tick_once(notes);
    }
    self.println(count);
  }

  fn tick_once(&mut self, notes: &Notes) {
    let other = self.clone();
    for (ind, window) in other.windows() {
      self.set(ind, notes.pick(&window));
    }
  }

  pub fn sum_plants(&self) -> i64 {
    self.iter().filter(|(_, p)| *p).map(|(i, _)| i).sum()
  }

  pub fn iter(&self) -> IndexPotIter { IndexPotIter::new(&self) }
  pub fn windows(&self) -> WindowPotIter { WindowPotIter::new(&self) }

  pub fn println(&self, t: usize) {
    print!("{:4} ", t);

    for i in -10 ..= 130 {
      if self.get(i) { print!("#") } else { print!("."); }
    }

    println!("");
  }

  fn left(&self) -> (usize, usize) {
    let true_left = (self.left_plant - self.left_ind) as usize;
    let first_seg = true_left / self.segment_length;
    let first_ind = true_left % self.segment_length;
    (first_seg, first_ind)
  }

  fn get(&self, ind: i64) -> bool {
    let true_ind: i64 = ind - self.left_ind;
    if true_ind < 0 ||
       true_ind >= (self.segments.len() * self.segment_length) as i64 {
      return false;
    }
    let true_ind = true_ind as usize;
    let seg = true_ind / self.segment_length;
    let seg_ind = true_ind % self.segment_length;
    self.segments[seg][seg_ind]
  }

  fn find_left(&self, mut ind: i64) -> i64 {
    while !self.get(ind) { ind += 1 }
    ind
  }

  fn find_right(&self, mut ind: i64) -> i64 {
    while !self.get(ind) { ind -= 1 }
    ind
  }

  fn set(&mut self, ind: i64, pick: bool) {
    let mut true_ind: i64 = ind - self.left_ind;
    if pick && true_ind < 0 {
      self.segments.insert(0, vec![false; self.segment_length]);
      self.left_ind -= self.segment_length as i64;
      true_ind += self.segment_length as i64;
    }
    if pick && true_ind >= (self.segments.len() * self.segment_length) as i64 {
      self.segments.push(vec![false; self.segment_length]);
    }
    if true_ind >= 0 {
      let true_ind = true_ind as usize;
      if true_ind < self.segments.len() * self.segment_length {
        let seg = true_ind / self.segment_length;
        let seg_ind = true_ind % self.segment_length;
        self.segments[seg][seg_ind] = pick;
      }
    }
    if pick {
      if ind < self.left_plant {
        self.left_plant = ind;
      }
      if ind > self.right_plant {
        self.right_plant = ind;
      }
    }
    if !pick {
      if ind == self.left_plant {
        self.left_plant = self.find_left(ind);
      }
      if ind == self.right_plant {
        self.right_plant = self.find_right(ind);
      }
    }
  }
}

pub struct Notes {
  patterns: [bool; 32]
}

impl Notes {
  pub fn from_iter<P: IntoIterator<Item = Pattern>>(pats: P) -> Notes {
    let mut patterns = [false; 32];
    for p in pats {
      patterns[p.index()] = p.result();
    }
    Notes { patterns }
  }

  pub fn pick(&self, key: &PatternKey) -> bool {
    self.patterns[key.index()]
  }
}

pub struct IndexPotIter<'a> {
  line: &'a PotLine,
  segment: usize,
  index: usize,
  pot_index: i64
}

impl<'a> Iterator for IndexPotIter<'a> {
  type Item = (i64, bool);

  fn next(&mut self) -> Option<(i64, bool)> {
    //  println!("nexting for {} ({} - {}) at {},{}",
    //           self.pot_index,
    //           self.line.left_plant,
    //           self.line.right_plant,
    //           self.segment,
    //           self.index,
    //  );
    if self.pot_index > self.line.right_plant {
      return None;
    }

    if self.segment >= self.line.segments.len() {
      panic!("Missing segments on the right {}/{}",
             self.segment, self.line.segments.len());
    }

    let r_pot_index = self.pot_index;
    let r_has_plant = (self.line.segments[self.segment])[self.index];

    self.pot_index += 1;
    self.index += 1;
    if self.index >= self.line.segments[self.segment].len() {
      self.index = 0;
      self.segment += 1;
    }
    Some((r_pot_index, r_has_plant))
  }
}

impl<'a> IndexPotIter<'a> {
  pub fn new(line: &'a PotLine) -> IndexPotIter<'a> {
    let (segment, index) = line.left();
    let pot_index = line.left_plant;
    IndexPotIter { line, segment, index, pot_index }
  }
}

pub struct WindowPotIter<'a> {
  iter: TailIter<'a>,
  prev_window: [bool; 5]
}

impl<'a> Iterator for WindowPotIter<'a> {
  type Item = (i64, PatternKey);

  fn next(&mut self) -> Option<(i64, PatternKey)> {
    self.iter.next().map(|(ind, val)| {
      let r_window = self.advance_window(val);
      (ind - 2, PatternKey::new(r_window))
    })
  }
}

impl<'a> WindowPotIter<'a> {
  pub fn new(line: &'a PotLine) -> WindowPotIter<'a> {
    WindowPotIter { iter: TailIter::new(line.iter()), prev_window: [false; 5] }
  }

  fn advance_window(&mut self, val: bool) -> [bool; 5] {
    let mut r_window = [false; 5];
    r_window[.. 4].copy_from_slice(&self.prev_window[1 ..]);
    r_window[4] = val;
    self.prev_window.copy_from_slice(&r_window[..]);
    r_window
  }
}

pub struct Pattern {
  key: PatternKey,
  result: bool
}

impl Pattern {
  pub fn from_line(line: &str) -> Pattern {
    let mut key_result = line.trim().split(" => ");
    let key = key_result.next().unwrap();
    let mut key_str = key.split("").filter(|c| !c.is_empty());
    let key: [bool; 5] = [
      key_str.next().unwrap() == "#",
      key_str.next().unwrap() == "#",
      key_str.next().unwrap() == "#",
      key_str.next().unwrap() == "#",
      key_str.next().unwrap() == "#"
    ];
    let result = key_result.next().unwrap() == "#";

    Pattern { key: PatternKey::new(key), result }
  }

  pub fn result(&self) -> bool { self.result }
  pub fn index(&self) -> usize { self.key.index() }
}

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct PatternKey {
  key: [bool; 5]
}

impl Debug for PatternKey {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.key.fmt(f) }
}

impl PatternKey {
  pub fn new(key: [bool; 5]) -> PatternKey { PatternKey { key } }
  pub fn index(&self) -> usize {
    PatternKey::bool_to_usize(self.key[0]) +
      PatternKey::bool_to_usize(self.key[1]) * 2 +
      PatternKey::bool_to_usize(self.key[2]) * 4 +
      PatternKey::bool_to_usize(self.key[3]) * 8 +
      PatternKey::bool_to_usize(self.key[4]) * 16
  }

  pub fn bool_to_usize(v: bool) -> usize { if v { 1 } else { 0 } }
}

pub enum TailIter<'a> {
  Iter(i64, IndexPotIter<'a>),
  Tail(i64, u8)
}

impl<'a> TailIter<'a> {
  pub fn new(iter: IndexPotIter<'a>) -> TailIter { TailIter::Iter(0, iter) }

  pub fn next(&mut self) -> Option<(i64, bool)> {
    match self {
      TailIter::Iter(i, iter) => {
        match iter.next() {
          Some((ni, v)) => {
            *i = ni;
            Some((ni, v))
          }
          None => {
            *i += 1;
            let i = *i;
            std::mem::replace(self, TailIter::Tail(i, 0));
            Some((i, false))
          }
        }
      }
      TailIter::Tail(i, count) => {
        *count += 1;
        *i += 1;
        if *count >= 4 { None } else { Some((*i, false)) }
      }
    }
  }
}
