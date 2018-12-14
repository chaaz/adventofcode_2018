pub struct Claim {
  pub id: u32,
  pub left: u32,
  pub top: u32,
  pub width: u32,
  pub height: u32
}

impl Claim {
  pub fn new(id: u32, left: u32, top: u32, width: u32, height: u32) -> Claim {
    Claim { id, left, top, width, height }
  }

  pub fn id(&self) -> u32 { self.id }
  pub fn top(&self) -> u32 { self.top }
  pub fn bottom(&self) -> u32 { self.top + self.height }
  pub fn left(&self) -> u32 { self.left }
  pub fn right(&self) -> u32 { self.left + self.width }
}
