use std::cmp::min;
use super::grammar::LineParser;

const WIDTH: u32 = 1000u32;
const SIZE: usize = (WIDTH * WIDTH) as usize;

pub fn run() {
  let content = include_str!("input.txt").trim();
  let mut grid = [0u8; SIZE];

  let (_, overlap) = content
    .split("\n")
    .map(LineParser::new().parse(line).unwrap())
    .fold((&mut grid, 0u32), |(grid, mut overlap), claim| {
      for y in claim.top() .. claim.bottom() {
        for x in claim.left() .. claim.right() {
          let ind = (x + WIDTH * y) as usize;
          grid[ind] = min(grid[ind] + 1, 3);
          if grid[ind] == 2 { overlap += 1; }
        }
      }

      (grid, overlap)
    });

  println!("overlap: {}", overlap);
}
