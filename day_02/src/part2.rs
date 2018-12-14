pub fn run() {
  let content = include_str!("input.txt");

  for l1 in content.split("\n") {
    for l2 in content.split("\n") {
      let misses = l1.chars().zip(l2.chars()).fold(0, |misses, (c1, c2)| {
        misses + rev_bool_to_int(c1 == c2)
      });

      if misses == 1 {
        let common: String = l1.chars().zip(l2.chars()).filter_map(|(c1, c2)| {
          if c1 == c2 { Some(c1) } else { None }
        }).collect();
        println!("{} vs {}\n{}", l1, l2, common);
      }
    }
  }
}

fn rev_bool_to_int(v: bool) -> u32 { if v { 0 } else { 1 } }
