// const PLAYERS: usize = 17;
// const HIGHEST: usize = 1104;

const PLAYERS: usize = 462;
const HIGHEST: usize = 7193800;

pub fn run() {
  // doesn't have to be have quite that capacity, technically
  let mut scores = vec![0usize; PLAYERS];

  let mut circle = Vec::with_capacity(HIGHEST + 1);
  circle.push(0usize);
  circle.push(1usize);
  let mut current = 1;

  for marble in 2usize ..= HIGHEST {
    if marble % 23 == 0 {
      let player = marble % scores.len();
      scores[player] += marble;
      let rem_ind = (current + circle.len() - 7) % circle.len();
      let rem_score = circle.remove(rem_ind);
      scores[player] += rem_score;
      current = rem_ind % circle.len();
    } else {
      let ins_pos = (current + 2) % circle.len();
      match ins_pos {
        0 => { circle.push(marble); current = circle.len() - 1; }
        _ => { circle.insert(ins_pos, marble); current = ins_pos; }
      }
    }
  }

  let (i, best) = scores.iter().enumerate().max_by_key(|(_, s)| *s).unwrap();
  println!("Player {} won with {}.", i, best);
}
