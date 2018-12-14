const TARGET: &str = "760221";
// const TARGET: &str = "51589";
// const TARGET: &str = "01245";
// const TARGET: &str = "59414";

pub fn run() {
  let mut recipes: Vec<u8> = vec![3, 7];
  let target: Vec<_> = TARGET.chars()
    .filter_map(|c| c.to_digit(10).map(|v| v as u8))
    .collect();
  let last_dig = target[target.len() - 1];

  let mut elf0: usize = 0;
  let mut elf1: usize = 1;
  loop {
    let sum = recipes[elf0] + recipes[elf1];
    if sum >= 10 {
      if push_and_check(sum / 10, last_dig, &mut recipes, &target) { break; }
    }
    if push_and_check(sum % 10, last_dig, &mut recipes, &target) { break; }

    elf0 = (elf0 + (recipes[elf0] as usize) + 1) % recipes.len();
    elf1 = (elf1 + (recipes[elf1] as usize) + 1) % recipes.len();
  }

  println!("Count before target: {}", recipes.len() - target.len());
}

fn push_and_check(d: u8, last_dig: u8, recipes: &mut Vec<u8>,
                  target: &[u8]) -> bool {
  recipes.push(d);
  d == last_dig && check_recipes(&recipes, &target)
}

fn check_recipes(recipes: &[u8], target: &[u8]) -> bool {
  recipes.len() >= target.len()
    && &recipes[recipes.len() - target.len() ..] == target
}
