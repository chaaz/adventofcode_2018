use std::cmp::Ordering;

pub fn main() {
  let map: Vec<_> =
    include_bytes!("input.txt").split(|c| *c == b'\n').collect();

  let mut carts = find_carts(&map);
  let mut t = 0;
  let (row, col) = loop {
    if let Tick::Crash(row, col) = carts.tick(&map) {
      break (row, col);
    }
    t += 1;
  };
  println!("Crash x,y at {},{} on {}", col, row, t);

}

fn find_carts(map: &Vec<&[u8]>) -> Carts {
  let mut carts = Carts::new();

  for (r, row) in map.iter().enumerate() {
    for (c, ch) in row.iter().enumerate() {
      match ch {
        b'>' => carts.add(r, c, Dir::new(0, 1)),
        b'<' => carts.add(r, c, Dir::new(0, -1)),
        b'^' => carts.add(r, c, Dir::new(-1, 0)),
        b'v' => carts.add(r, c, Dir::new(1, 0)),
        _ => ()
      }
    }
  }

  carts
}

struct Carts {
  carts: Vec<Cart>
}

impl Carts {
  pub fn new() -> Carts { Carts { carts: Vec::new() } }

  pub fn add(&mut self, row: usize, col: usize, dir: Dir) {
    self.carts.push(Cart::new(row, col, dir));
  }

  pub fn tick(&mut self, map: &Vec<&[u8]>) -> Tick {
    self.sort_carts();
    for i in 0 .. self.carts.len() {
      let (row, col) = {
        let cart = self.carts.get_mut(i).unwrap();
        cart.tick();
        (cart.row, cart.col)
      };
      if self.is_crashed(i, row, col) {
        return Tick::Crash(row, col);
      }
      let cart = self.carts.get_mut(i).unwrap();
      cart.update(map);
    }
    Tick::Ok
  }

  fn sort_carts(&mut self) {
    self.carts.sort_by(|c0, c1| {
      if c0.row < c1.row { Ordering::Less }
      else if c0.row > c1.row { Ordering::Greater }
      else if c0.col < c1.col { Ordering::Less }
      else if c0.col > c1.col { Ordering::Greater }
      else { panic!("A collision has already occured.") }
    });
  }

  fn is_crashed(&self, cart_ind: usize, row: usize, col: usize) -> bool {
    for (i, cart) in self.carts.iter().enumerate() {
      if i == cart_ind { continue; }
      if row == cart.row && col == cart.col { 
        return true;
      }
    }
    false
  }
}

struct Cart {
  row: usize,
  col: usize,
  dir: Dir,
  mode: Mode
}

impl Cart {
  pub fn new(row: usize, col: usize, dir: Dir) -> Cart {
    Cart { row, col, dir, mode: Mode::Left }
  }

  pub fn tick(&mut self) {
    self.row = (self.row as i32 + self.dir.row) as usize;
    self.col = (self.col as i32 + self.dir.col) as usize;
  }

  pub fn update(&mut self, map: &Vec<&[u8]>) {
    match map[self.row][self.col] {
      b'/' => self.dir.slash1(),
      b'\\' => self.dir.slash2(),
      b'+' => self.dir.turn(self.mode.adv()),
      _ => ()
    }
  }
}

struct Dir {
  row: i32,
  col: i32
}

impl Dir {
  pub fn new(row: i32, col: i32) -> Dir { Dir { row, col } }

  pub fn slash1(&mut self) {
    let (r, c) = (self.row, self.col);
    self.row = -c;
    self.col = -r;
  }

  pub fn slash2(&mut self) {
    let (r, c) = (self.row, self.col);
    self.row = c;
    self.col = r;
  }

  pub fn turn(&mut self, mode: Mode) {
    let r = self.row;
    let c = self.col;
    match mode {
      Mode::Straight => (),
      Mode::Left => { self.row = -c; self.col = r; }
      Mode::Right => { self.row = c; self.col = -r; }
    }
  }
}

#[derive(Clone)]
enum Mode {
  Left,
  Straight,
  Right
}

impl Mode {
  pub fn adv(&mut self) -> Mode {
    let new = match self {
      Mode::Left => Mode::Straight,
      Mode::Straight => Mode::Right,
      Mode::Right => Mode::Left
    };
    std::mem::replace(self, new)
  }
}

enum Tick {
  Ok,
  Crash(usize, usize),
}
