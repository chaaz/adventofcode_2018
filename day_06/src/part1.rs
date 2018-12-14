use std::ops::RangeInclusive;
use std::cmp::Ordering;

pub fn run() {
  let content = include_str!("input.txt").trim().split("\n");

  let points: Vec<_> = content.map(|line| Point::from_line(line)).collect();
  let rect = Rect::min_bounds(&points);

  let mut total = vec![(0u32, false); points.len()]; // (area, infinite)
  find_totals(&points, &rect, &mut total);
  let best = total.iter().enumerate().max_by(|(_, (a0, i0)), (_, (a1, i1))| {
    match (i0, i1) {
      (true, true) => Ordering::Equal,
      (true, false) => Ordering::Less,
      (false, true) => Ordering::Greater,
      _ => a0.cmp(a1)
    }
  }).unwrap();

  println!("best: ({}) = {}", best.0, (best.1).0);
}

fn find_totals(pts: &Vec<Point>, rect: &Rect, totals: &mut Vec<(u32, bool)>) {
  for y in rect.y_range() {
    for x in rect.x_range() {
      let (mut min_dist, mut min_i, mut tie) = (std::u32::MAX, 0, false);
      for i in 0 .. pts.len() {
        let pt = &pts[i];
        let dist = pt.dist(x, y);
        if dist < min_dist {
          min_dist = dist;
          min_i = i;
          tie = false;
        } else if dist == min_dist {
          tie = true;
        }
      }
      if !tie {
        let infinite = rect.is_boundry(x, y);
        totals[min_i] = (totals[min_i].0 + 1, totals[min_i].1 || infinite);
      }
    }
  }
}

struct Point {
  x: u32,
  y: u32
}

impl Point {
  pub fn from_line(line: &str) -> Point {
    let mut xy = line.split(",");
    let x = xy.next().unwrap().trim().parse().unwrap();
    let y = xy.next().unwrap().trim().parse().unwrap();
    Point { x, y }
  }

  pub fn dist(&self, x: u32, y: u32) -> u32 {
    let dist_x = if x > self.x { x - self.x } else { self.x - x };
    let dist_y = if y > self.y { y - self.y } else { self.y - y };
    dist_x + dist_y
  }
}

struct Rect {
  min_x: u32,
  min_y: u32,
  max_x: u32,
  max_y: u32
}

impl Rect {
  pub fn min_bounds(points: &Vec<Point>) -> Rect {
    let mut min_x: u32 = std::u32::MAX;
    let mut min_y: u32 = std::u32::MAX;
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;

    for pt in points {
      if pt.x < min_x { min_x = pt.x; }
      if pt.y < min_y { min_y = pt.y; }
      if pt.x > max_x { max_x = pt.x; }
      if pt.y > max_y { max_y = pt.y; }
    }

    Rect { min_x, min_y, max_x, max_y }
  }

  pub fn x_range(&self) -> RangeInclusive<u32> { self.min_x ..= self.max_x }
  pub fn y_range(&self) -> RangeInclusive<u32> { self.min_y ..= self.max_y }

  pub fn is_boundry(&self, x: u32, y: u32) -> bool {
    x == self.min_x || x == self.max_x || y == self.min_y || y == self.max_y
  }
}
