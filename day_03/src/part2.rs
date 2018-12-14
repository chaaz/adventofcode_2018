use std::cmp::min;
use super::grammar::LineParser;

const WIDTH: u32 = 1000u32;
const SIZE: usize = (WIDTH * WIDTH) as usize;

pub fn run() {
  let content = include_str!("input.txt").trim();
  let mut grid = [0u8; SIZE];

  let grid = content
    .split("\n")
    .map(|line| LineParser::new().parse(line).unwrap())
    .fold(&mut grid, |grid, claim| {
      for y in claim.top() .. claim.bottom() {
        for x in claim.left() .. claim.right() {
          let ind = (x + WIDTH * y) as usize;
          grid[ind] = min(grid[ind] + 1, 3);
        }
      }
      grid
    });

  let mut separated = content
    .split("\n")
    .map(|line| LineParser::new().parse(line).unwrap())
    .filter_map(|claim| {
      for y in claim.top() .. claim.bottom() {
        for x in claim.left() .. claim.right() {
          let ind = (x + WIDTH * y) as usize;
          match grid[ind] {
            0 => panic!("Unpossible!"),
            1 => (),
            _ => return None
          }
        }
      }
      Some(claim)
    });

  let winner = separated.next().unwrap();
  assert!(separated.next().is_none());

  println!("Winner: {}", winner.id());
}
