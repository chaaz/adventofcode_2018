const SIZE: usize = 300;
const LEN: usize = SIZE * SIZE;
const SERIAL: usize = 5034;
// const SERIAL: usize = 18;
// const SERIAL: usize = 42;

pub fn run() {
  let mut grid = [0i8; LEN];
  fill_grid(&mut grid);

  let mut max_val = std::i32::MIN;
  let mut max_x = 0usize;
  let mut max_y = 0usize;
  let mut max_s = 0usize;
  // part2 : for s in 0 .. SIZE
  for s in 2 .. 3 {
    println!("checking s {}", s);
    for y in 0 .. SIZE - s {
      let mut prev_level = None;
      for x in 0 .. SIZE - s {
        let level = level(&grid, x, y, s, prev_level);
        if level > max_val {
          max_val = level;
          max_x = x;
          max_y = y;
          max_s = s;
        }
        prev_level = Some(level);
      }
    }
  }

  println!("max is {} at ({},{},{}).",
           max_val, max_x + 1, max_y + 1, max_s + 1);
}

fn level(
  grid: &[i8; LEN], x: usize, y: usize, s: usize, prev: Option<i32>
) -> i32 {
  if let Some(prev) = prev {
    let mut total = prev;
    for yv in y ..= y + s {
      total -= grid[x - 1 + yv * SIZE] as i32;
      total += grid[x + s + yv * SIZE] as i32;
    }
    total
  } else {
    let mut total = 0i32;
    for yv in y ..= y + s {
      for xv in x ..= x + s {
        total += grid[xv + yv * SIZE] as i32;
      }
    }
    total
  }
}

fn fill_grid(grid: &mut [i8; LEN]) {
  for y in 0 .. SIZE {
    for x in 0 .. SIZE {
      grid[x + y * SIZE] = calc(x, y);
    }
  }
}

fn calc(x: usize, y: usize) -> i8 {
  let x = x + 1;
  let y = y + 1;

  let rack_id = x + 10;
  let start = rack_id * y;
  let added = start + SERIAL;
  let multd = added * rack_id;
  let hdig = digit_hundred(multd);
  hdig - 5i8
}

fn digit_hundred(val: usize) -> i8 {
  ((val / 100usize) % 10) as i8
}
