use std::collections::HashMap;

pub fn run() {
  let content = include_str!("input.txt");

  let (twos, threes) = content
    .split("\n")
    .map(|line| {
      let map = line.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0u32) += 1;
        map
      });
      (map.values().any(|v| *v == 2), map.values().any(|v| *v == 3))
    })
    .fold((0, 0), |(twos, threes), (has2, has3)| {
      (twos + bool_to_int(has2), threes + bool_to_int(has3))
    });

  println!("twos: {}, threes: {}, prod: {}", twos, threes, twos * threes);
}

fn bool_to_int(v: bool) -> u32 { if v { 1 } else { 0 } }
