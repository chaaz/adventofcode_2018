pub fn run() {
  let content: Vec<u32> = include_str!("input.txt")
    .trim()
    .split(" ")
    .map(|v| v.parse().unwrap())
    .collect();

  let meta_sum = sum_node_meta(&content[..]);
  println!("sum of meta: {}", meta_sum.0);
}

fn sum_node_meta(content: &[u32]) -> (u32, u32) {
  let childs = content[0];
  let metas = content[1];

  let mut sum = 0u32;
  let mut start = 2u32;
  for _ in 0 .. childs {
    let (sub_sum, sub_len) = sum_node_meta(&content[start as usize ..]);
    sum += sub_sum;
    start += sub_len;
  }

  for _ in 0 .. metas {
    sum += content[start as usize];
    start += 1;
  }

  (sum, start)
}
