use std::fmt;

#[derive(Debug)]
struct Color(u8, u8, u8);

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Color(r, g, b) = self;
    write!(f, "rgb({r}, {g}, {b})")?;
    write!(f, " #{r:02x}{g:02x}{b:02x}")
  }
}

fn main() {

  // RGB (128, 255, 90) 0x80FF5A
  // RGB (0, 3, 254) 0x0003FE
  // RGB (0, 0, 0) 0x000000

  for color in [Color(128, 255, 90), Color(0, 3, 254), Color(0, 0, 0)] {
    println!("{color:?}");
    println!("{color}");
  }
}
