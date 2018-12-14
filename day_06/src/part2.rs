use std::ops::RangeInclusive;
pub fn run() {
  let content = include_str!("input.txt").trim().split("\n");
  let extent: i32 = 10_000;

  let points: Vec<_> = content.map(|line| Point::from_line(line)).collect();
  let rect = Rect::min_extent(&points, extent);

  let region_size = find_totals_lt(&points, &rect, extent);
  println!("size: {}", region_size);
}

fn find_totals_lt(pts: &Vec<Point>, rect: &Rect, extent: i32) -> i32 {
  let mut size = 0i32;
  let grid_size = rect.grid_size();
  let mut done = 0;
  for y in rect.y_range() {
    for x in rect.x_range() {
      let mut total = 0i32;
      for i in 0 .. pts.len() {
        let pt = &pts[i];
        total += pt.dist(x, y);
      }
      if total < extent {
        if rect.is_boundry(x, y) {
          panic!("Region on boundry: larger extent required");
        }
        size += 1;
      }
      done += 1;
      if done % 1000000 == 0 {
        println!("done: {}/{} ({:2.1}%)", done, grid_size, 
                 (done as f32) / (grid_size as f32) * 100f32);
      }
    }
  }

  size
}

struct Point {
  x: i32,
  y: i32
}

impl Point {
  pub fn from_line(line: &str) -> Point {
    let mut xy = line.split(",");
    let x = xy.next().unwrap().trim().parse().unwrap();
    let y = xy.next().unwrap().trim().parse().unwrap();
    Point { x, y }
  }

  pub fn dist(&self, x: i32, y: i32) -> i32 {
    let dist_x = if x > self.x { x - self.x } else { self.x - x };
    let dist_y = if y > self.y { y - self.y } else { self.y - y };
    dist_x + dist_y
  }
}

struct Rect {
  min_x: i32,
  min_y: i32,
  max_x: i32,
  max_y: i32
}

impl Rect {
  pub fn min_extent(points: &Vec<Point>, _ext: i32) -> Rect {
    let mut min_x: i32 = std::i32::MAX;
    let mut min_y: i32 = std::i32::MAX;
    let mut max_x: i32 = std::i32::MIN;
    let mut max_y: i32 = std::i32::MIN;

    for pt in points {
      if pt.x < min_x { min_x = pt.x; }
      if pt.y < min_y { min_y = pt.y; }
      if pt.x > max_x { max_x = pt.x; }
      if pt.y > max_y { max_y = pt.y; }
    }

    // try just with minimum bounds for now
    Rect { min_x, min_y, max_x, max_y }
  }

  pub fn x_range(&self) -> RangeInclusive<i32> { self.min_x ..= self.max_x }
  pub fn y_range(&self) -> RangeInclusive<i32> { self.min_y ..= self.max_y }

  pub fn is_boundry(&self, x: i32, y: i32) -> bool {
    x == self.min_x || x == self.max_x || y == self.min_y || y == self.max_y
  }

  pub fn grid_size(&self) -> i32 {
    (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
  }
}
