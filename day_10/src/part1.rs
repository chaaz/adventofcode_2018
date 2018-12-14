pub fn run() {
  let content = include_str!("input.txt").trim().split("\n");
  let mut lights: Vec<_> = content.map(Light::from_line).collect();
  loop_until_minimum(&mut lights);
}

fn loop_until_minimum(lights: &mut Vec<Light>) -> u32 {
  let mut t = 0u32;
  let mut last_width = std::i32::MAX;
  loop {
    lights.iter_mut().for_each(|l| l.tick());
    t += 1;
    let width = get_width(lights, t);
    if width > last_width {
      lights.iter_mut().for_each(|l| l.untick());
      t -= 1;
      break;
    }
    last_width = width;
    print_lights(&lights, t);
  }
  t
}

fn print_lights(lights: &Vec<Light>, t: u32) {
  let minx = lights.iter().min_by_key(|l| l.px).unwrap().px;
  let maxx = lights.iter().max_by_key(|l| l.px).unwrap().px;
  let w = maxx - minx;

  let miny = lights.iter().min_by_key(|l| l.py).unwrap().py;
  let maxy = lights.iter().max_by_key(|l| l.py).unwrap().py;
  let h = maxy - miny;

  if w > 120 { return; }
  if h > 40 { return; }

  println!("t: {}", t);

  for iy in miny ..= maxy {
    let y = maxy - iy + miny;
    for x in minx ..= maxx {
      let hit = lights.iter().any(|l| l.px == x && l.py == y);
      if hit { print!("#"); } else { print!("."); }
    }
    println!("");
  }
}

fn get_width(lights: &Vec<Light>, _t: u32) -> i32 {
  let minx = lights.iter().min_by_key(|l| l.px).unwrap().px;
  let maxx = lights.iter().max_by_key(|l| l.px).unwrap().px;
  maxx - minx
}

struct Light {
  px: i32,
  py: i32,
  vx: i32,
  vy: i32
}

impl Light {
  pub fn from_line(line: &str) -> Light {
    let pos = line.split("<").nth(1).unwrap().split(">").next().unwrap();
    let mut pxy = pos.split(",").map(|v| v.trim().parse().unwrap());
    let px = pxy.next().unwrap();
    let py = pxy.next().unwrap();

    let vel = line.split("<").nth(2).unwrap().split(">").next().unwrap();
    let mut vxy = vel.split(",").map(|v| v.trim().parse().unwrap());
    let vx = vxy.next().unwrap();
    let vy = vxy.next().unwrap();

    Light { px, py, vx, vy }
  }

  fn tick(&mut self) {
    self.px += self.vx;
    self.py += self.vy;
  }

  fn untick(&mut self) {
    self.px -= self.vx;
    self.py -= self.vy;
  }
}
