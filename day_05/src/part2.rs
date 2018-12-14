pub fn run() {
  let bytes = include_bytes!("input.txt");
  let mut polymer = Vec::new();
  polymer.extend_from_slice(&bytes[..]);
  polymer.pop();    // remove trailing '\n'
  let tables = CaseTables::new();

  let best = (b'a' ..= b'z').into_iter().map(|problem| {
    let better_poly: Vec<u8> = polymer.clone().into_iter().filter(|b| {
      *b != problem && *b != tables.flip(problem)
    }).collect();
    (reacted(&tables, &better_poly).len(), problem as char)
  }).min_by_key(|(b, _)| *b).unwrap();

  println!("Overall best: {} ({})", best.0, best.1);
}

fn reacted(tables: &CaseTables, polymer: &[u8]) -> Vec<u8> {
  let mut reacted = Vec::new();
  for unit in polymer {
    if reacted.is_empty() || *unit != tables.flip(reacted[reacted.len() - 1]) {
      reacted.push(*unit);
    } else {
      reacted.pop();
    }
  }
  reacted
}

struct CaseTables {
  table: [u8; 255],
}

impl CaseTables {
  pub fn new() -> CaseTables {
    let mut table = [0u8; 255];

    for lower in b'a' ..= b'z' {
      let upper = (lower as char).to_uppercase().next().unwrap();
      let mut byte_upper = [0u8; 1];
      upper.encode_utf8(&mut byte_upper);
      let upper = byte_upper[0];

      table[upper as usize] = lower;
      table[lower as usize] = upper;
    }

    CaseTables { table }
  }

  pub fn flip(&self, u: u8) -> u8 { self.table[u as usize] }
}
