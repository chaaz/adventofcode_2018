use std::cmp::Ordering;

pub fn run() {
  let content = include_bytes!("input.txt");

  let (tracks, mut carts) = parse_all(content);

  let mut t = 0;
  let (row, col) = loop {
    if let Tick::Solo(row, col) = carts.tick(&tracks) {
      break (row, col);
    }
    t += 1;
  };

  println!("Solo x,y at {},{} on {}", col, row, t);
}

fn parse_all(map: &[u8]) -> (Tracks, Carts) {
  let mut tracks = Tracks::new();
  let mut carts = Carts::new();

  for (row, line) in map.split(|b| *b == b'\n').enumerate() {
    for (col, c) in line.into_iter().enumerate() {
      match c {
        b'/' => tracks.curve1(row, col),
        b'\\' => tracks.curve2(row, col),
        b'-' => tracks.horiz(row, col),
        b'|' => tracks.vert(row, col),
        b'+' => tracks.inter(row, col),
        b'>' => {
          tracks.horiz(row, col);
          let track = tracks.find(row, col);
          carts.cart(row, col, Dir::Right, track);
        }
        b'<' => {
          tracks.horiz(row, col);
          let track = tracks.find(row, col);
          carts.cart(row, col, Dir::Left, track);
        }
        b'^' => {
          tracks.vert(row, col);
          let track = tracks.find(row, col);
          carts.cart(row, col, Dir::Up, track);
        }
        b'v' => {
          tracks.vert(row, col);
          let track = tracks.find(row, col);
          carts.cart(row, col, Dir::Down, track);
        }
        b' ' => (),
        u => panic!("Unknown character {}", u)
      }
    }
  }

  (tracks, carts)
}

pub struct Tracks {
  loops: Vec<Loop>,
}

impl Tracks {
  pub fn new() -> Tracks {
    Tracks { loops: Vec::new() }
  }

  pub fn find(&self, row: usize, col: usize) -> usize {
    self.loops.iter().enumerate()
      .find(|(_, t)| t.is_on(row, col))
      .map(|(i, _)| i).expect(&format!("no loop on {},{}", row, col))
  }

  pub fn curve1(&mut self, row: usize, col: usize) {
    if !self.expect_br(row, col) {
      self.loops.push(Loop::new(row, col));
    }
  }

  pub fn curve2(&mut self, row: usize, col: usize) {
    if let Some(l) = self.expect_tr(row, col) {
      l.expand_right(col);
      l.close_right();
    } else if let Some(l) = self.expect_bl(row, col) {
      l.expand_down(row);
      l.close();
    } else {
      panic!("Unexpected curve2: {}, {}", row, col);
    }
  }

  pub fn horiz(&mut self, row: usize, col: usize) {
    if let Some((l, top)) = self.expect_h(row, col) {
      if top { l.expand_right(col); }
    } else {
      panic!("Unexpected horiz: {}, {}", row, col);
    }
  }

  pub fn vert(&mut self, row: usize, col: usize) {
    if let Some((l, left)) = self.expect_v(row, col) {
      if left { l.expand_down(row); }
    } else {
      panic!("Unexpected vert: {}, {}", row, col);
    }
  }

  pub fn inter(&mut self, row: usize, col: usize) {
    if let Some((l0, top)) = self.expect_h(row, col) {
      if top { l0.expand_right(col); }
      if let Some((l1, left)) = self.expect_v(row, col) {
        if left { l1.expand_down(row); }
      } else {
        panic!("Unexpected vert inter: {}, {}", row, col);
      }
    } else {
      panic!("Unexpected horiz inter: {}, {}", row, col);
    }
  }

  fn expect_br(&self, row: usize, col: usize) -> bool {
    self.loops.iter().any(|l| {
      l.bottom == row && l.right == col && l.closed && l.closed_right
    })
  }

  fn expect_tr(&mut self, row: usize, col: usize) -> Option<&mut Loop> {
    self.loops.iter_mut().find(|l| {
      l.top == row && l.right == col - 1 && !l.closed && !l.closed_right
    })
  }

  fn expect_bl(&mut self, row: usize, col: usize) -> Option<&mut Loop> {
    self.loops.iter_mut().find(|l| {
      l.bottom == row - 1 && l.left == col && !l.closed && l.closed_right
    })
  }

  fn expect_h(&mut self, row: usize, col: usize) -> Option<(&mut Loop, bool)> {
    self.loops
      .iter_mut()
      .filter_map(|l| {
        if l.top == row && l.right == col - 1 && !l.closed && !l.closed_right {
          Some((l, true))
        } else if l.bottom == row && l.left < col && l.right > col
                  && l.closed && l.closed_right {
          Some((l, false))
        } else {
          None
        }
      })
      .next()
  }

  fn expect_v(&mut self, row: usize, col: usize) -> Option<(&mut Loop, bool)> {
    self.loops
      .iter_mut()
      .filter_map(|l| {
        if l.bottom == row - 1 && l.left == col && !l.closed && l.closed_right {
          Some((l, true))
        } else if l.bottom == row && l.right == col && !l.closed
                  && l.closed_right {
          Some((l, false))
        } else {
          None
        }
      })
      .next()
  }
}

#[derive(Debug)]
struct Loop {
  left: usize,
  right: usize,
  top: usize,
  bottom: usize,
  closed: bool,
  closed_right: bool
}

impl Loop {
  pub fn new(top: usize, left: usize) -> Loop {
    let closed = false;
    let closed_right = false;
    Loop { left, right: left, top, bottom: top, closed, closed_right }
  }

  pub fn is_on(&self, row: usize, col: usize) -> bool {
    ((row == self.top || (row == self.bottom && self.closed)) &&
      (col >= self.left && col <= self.right)) ||
    ((row >= self.top && row <= self.bottom) &&
      (col == self.left || (col == self.right && self.closed_right)))
  }

  pub fn expand_down(&mut self, row: usize) {
    if self.closed { panic!("Can't expand down: closed."); }
    self.bottom = row;
  }

  pub fn expand_right(&mut self, col: usize) {
    if self.closed { panic!("Can't expand right: closed."); }
    if self.closed_right { panic!("Can't expand right: closed_right."); }
    self.right = col;
  }

  pub fn close(&mut self) {
    self.closed = true;
  }

  pub fn close_right(&mut self) {
    self.closed_right = true;
  }
}

struct Carts {
  carts: Vec<Cart>
}

impl Carts {
  pub fn new() -> Carts { Carts { carts: Vec::new() } }

  pub fn cart(&mut self, row: usize, col: usize, dir: Dir, track: usize) {
    self.carts.push(Cart::new(row, col, dir, track));
  }

  pub fn tick(&mut self, tracks: &Tracks) -> Tick {
    self.carts.sort_by(|c0, c1| {
      if c0.row < c1.row { Ordering::Less }
      else if c0.row > c1.row { Ordering::Greater }
      else if c0.col < c1.col { Ordering::Less }
      else if c0.col > c1.col { Ordering::Greater }
      else { panic!("A collision has already occured.") }
    });

    let mut i = 0;
    while i < self.carts.len() {
      let (row, col) = {
        let cart = self.carts.get_mut(i).unwrap();
        let (dy, dx) = cart.dir.velocity();
        cart.row = (cart.row as isize + dy) as usize;
        cart.col = (cart.col as isize + dx) as usize;
        (cart.row, cart.col)
      };
      if let Some(j) = self.is_crashed(i, row, col) {
        println!("Would-be crash at {},{}", row, col);
        if i < j {
          self.carts.remove(j);
          self.carts.remove(i);
        } else {
          self.carts.remove(i);
          self.carts.remove(j);
          i -= 1;
        }
        if self.carts.is_empty() {
          panic!("No carts left!");
        }
      } else {
        let cart = self.carts.get_mut(i).unwrap();
        if let Some(dir) = cart.corner_turn(&tracks) { cart.dir = dir; }
        else if let Some((dir, mode, track)) = cart.inter_turn(tracks) {
          cart.dir = dir;
          cart.track = track;
          cart.mode = mode;
        }
        i += 1;
      }
    }
    if self.carts.len() == 1 {
      let cart = &self.carts[0];
      Tick::Solo(cart.row, cart.col)
    } else {
      Tick::Ok
    }
  }

  fn is_crashed(&self, cart_ind: usize, row: usize, col: usize)
  -> Option<usize> {
    for (i, cart) in self.carts.iter().enumerate() {
      if i == cart_ind { continue; }
      if row == cart.row && col == cart.col { 
        return Some(i);
      }
    }
    None
  }
}

struct Cart {
  row: usize,
  col: usize,
  dir: Dir,
  mode: Mode,
  track: usize
}

impl Cart {
  pub fn new(row: usize, col: usize, dir: Dir, track: usize) -> Cart {
    Cart { row, col, dir, mode: Mode::Left, track }
  }

  pub fn corner_turn(&self, tracks: &Tracks) -> Option<Dir> {
    let track = &tracks.loops[self.track];
    if self.row == track.top {
      if self.col == track.left {
        match &self.dir {
          Dir::Left => Some(Dir::Down),
          Dir::Up => Some(Dir::Right),
          d => panic!("Wrong dir {:?} for {},{}", d, self.row, self.col)
        }
      } else if self.col == track.right {
        match &self.dir {
          Dir::Right => Some(Dir::Down),
          Dir::Up => Some(Dir::Left),
          d => panic!("Wrong dir {:?} for {},{}", d, self.row, self.col)
        }
      } else {
        None
      }
    } else if self.row == track.bottom {
      if self.col == track.left {
        match &self.dir {
          Dir::Left => Some(Dir::Up),
          Dir::Down => Some(Dir::Right),
          d => panic!("Wrong dir {:?} for {},{}", d, self.row, self.col)
        }
      } else if self.col == track.right {
        match &self.dir {
          Dir::Right => Some(Dir::Up),
          Dir::Down => Some(Dir::Left),
          d => panic!("Wrong dir {:?} for {},{}", d, self.row, self.col)
        }
      } else {
        None
      }
    } else if self.col != track.left && self.col != track.right {
      panic!("Cart at {},{} is not on track {}.",
             self.row, self.col, self.track)
    } else {
      None
    }
  }

  pub fn inter_turn(&self, tracks: &Tracks) -> Option<(Dir, Mode, usize)> {
    for (i, track) in tracks.loops.iter().enumerate() {
      if self.track == i { continue; }
      if track.is_on(self.row, self.col) {
        let (dir, mode, track) = match &self.mode {
          m @ Mode::Left => (self.dir.left(), m.next(), i),
          m @ Mode::Straight => (self.dir.stay(), m.next(), self.track),
          m @ Mode::Right => (self.dir.right(), m.next(), i)
        };
        return Some((dir, mode, track));
      }
    }
    None
  }
}

#[derive(Debug)]
enum Dir {
  Up,
  Down,
  Left,
  Right
}

impl Dir {
  fn velocity(&self) -> (isize, isize) {
    match self {
      Dir::Up => (-1, 0),
      Dir::Down => (1, 0),
      Dir::Left => (0, -1),
      Dir::Right => (0, 1)
    }
  }

  fn left(&self) -> Dir {
    match self {
      Dir::Up => Dir::Left,
      Dir::Down => Dir::Right,
      Dir::Left => Dir::Down,
      Dir::Right => Dir::Up
    }
  }

  fn right(&self) -> Dir {
    match self {
      Dir::Up => Dir::Right,
      Dir::Down => Dir::Left,
      Dir::Left => Dir::Up,
      Dir::Right => Dir::Down
    }
  }

  fn stay(&self) -> Dir {
    match self {
      Dir::Up => Dir::Up,
      Dir::Down => Dir::Down,
      Dir::Left => Dir::Left,
      Dir::Right => Dir::Right
    }
  }
}

enum Mode {
  Left,
  Straight,
  Right
}

impl Mode {
  fn next(&self) -> Mode {
    match self {
      Mode::Left => Mode::Straight,
      Mode::Straight => Mode::Right,
      Mode::Right => Mode::Left
    }
  }
}

enum Tick {
  Ok,
  Solo(usize, usize),
}
