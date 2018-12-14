const TARGET: usize = 760221;
// const TARGET: usize = 9;
// const TARGET: usize = 2018;

pub fn run() {
  let mut recipes: Vec<u8> = vec![3, 7];

  let mut elf0: usize = 0;
  let mut elf1: usize = 1;

  loop {
    let sum = recipes[elf0] + recipes[elf1];
    if sum >= 10 { recipes.push(sum / 10); }
    recipes.push(sum % 10);
    if recipes.len() >= TARGET + 10 { break; }
    elf0 = (elf0 + (recipes[elf0] as usize) + 1) % recipes.len();
    elf1 = (elf1 + (recipes[elf1] as usize) + 1) % recipes.len();
  }

  print!("Last 10 recipes: ");
  for i in TARGET .. TARGET + 10 {
    print!("{}", recipes[i]);
  }
  println!("");
}
