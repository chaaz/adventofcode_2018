use std::collections::HashMap;
use itertools::Itertools;

pub fn run() {
  let content = include_str!("input.txt").trim();
  let mut lines: Vec<_> = content.split("\n").collect();
  // yyyy-MM-dd HH:mm => lexi sort = chrono sort
  lines.sort();

  let mut current_night = 0u32;
  let night_lines = lines.into_iter().group_by(|line| {
    if line.contains("Guard #") {
      current_night += 1;
    }
    current_night
  });
  let nights = night_lines.into_iter().map(|(_, g)| Night::from_lines(g));

  let guards = nights.into_iter().fold(HashMap::new(), |mut guards, night| {
    let id = night.id.clone();
    guards.entry(id.clone()).or_insert(Guard::new(id)).add_night(night);
    guards
  });
  let guards = guards.values();
  let best_guard = guards.max_by_key(|g| g.best_minute().1).unwrap();
  let best_minute = best_guard.best_minute().0;
  let id: usize = best_guard.id().parse().unwrap();

  println!("guard {} * minute {} = {}", id, best_minute, id * best_minute);
}

struct Guard {
  id: String,
  nights: Vec<Night>
}

impl Guard {
  pub fn new(id: String) -> Guard { Guard { id, nights: Vec::new() } }
  pub fn add_night(&mut self, night: Night) { self.nights.push(night) }
  pub fn id(&self) -> &str { &self.id }

  pub fn best_minute(&self) -> (usize, u32) {
    let mut minutes = vec![0u32; 60];
    for night in &self.nights {
      for sleep in &night.sleeps {
        for minute in sleep.start .. sleep.end {
          minutes[minute as usize] += 1;
        }
      }
    }

    let best = minutes.iter().enumerate().max_by_key(|(_, m)| *m);
    best.map(|(i, m)| (i, *m)).unwrap()
  }
}

struct Night {
  id: String,
  sleeps: Vec<Sleep>
}

impl Night {
  pub fn from_lines<'a, I: Iterator<Item = &'a str>>(mut i: I) -> Night {
    let duty = i.next().unwrap();
    let id = duty.split("Guard #").nth(1).unwrap().split(" ").next().unwrap();
    let sleeps = i.chunks(2).into_iter().map(Sleep::from_chunk).collect();
    Night { id: id.to_string(), sleeps }
  }
}

struct Sleep {
  start: u8,
  end: u8
}

impl Sleep {
  pub fn from_chunk<'a, I: Iterator<Item = &'a str>>(mut chunk: I) -> Sleep {
    let start = chunk.next().unwrap();
    let end = chunk.next().unwrap();
    assert!(start.contains("falls asleep"), "not sleep: {}", start);
    assert!(end.contains("wakes up"), "not wakes: {}", end);
    Sleep::new(Sleep::parse_minute(start), Sleep::parse_minute(end))
  }

  pub fn new(start: u8, end: u8) -> Sleep { Sleep { start, end } }

  fn parse_minute(line: &str) -> u8 {
    let minute = line.split("]").next().unwrap().split(":").nth(1).unwrap();
    minute.parse().unwrap()
  }
}
