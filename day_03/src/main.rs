mod part2;
mod claim;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

fn main() {
  part2::run()
}
