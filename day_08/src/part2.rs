pub fn run() {
  let content: Vec<u32> = include_str!("input.txt")
    .trim()
    .split(" ")
    .map(|v| v.parse().unwrap())
    .collect();

  let value_sum = sum_node_value(&content[..]);
  println!("sum of value: {}", value_sum.0);
}

fn sum_node_value(content: &[u32]) -> (u32, u32) {
  let childs = content[0];
  let metas = content[1];

  let start = 2u32;
  let mut child_values = Vec::new();
  let start = (0 .. childs).fold((start, &mut child_values), |(start, cv), _| {
    let (sum, len) = sum_node_value(&content[start as usize ..]);
    cv.push(sum);
    (start + len, cv)
  }).0;

  let value = match childs {
    0 => (0 .. metas).fold(0, |total, m| total + content[(start + m) as usize]),
    _ => (0 .. metas).fold(0, |total, m| {
      let i = content[(start + m) as usize];
      total + match i {
        0 => 0,
        _ => child_values.get((i - 1) as usize).cloned().unwrap_or(0)
      }
    })
  };

  (value, start + metas)
}
