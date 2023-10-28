use std::fmt;
use std::vec;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[")?;

    for (i, x) in self.0.iter().enumerate() {
      write!(f, "{i}: {x}")?;
      if i != self.0.len() - 1 { write!(f, ", ")?; }
    }

    write!(f, "]")
  }
}

fn main() {

  let list = List(vec!(1,2,3,4));
  println!("Vector formatting:");
  println!("Debug: {list:?}");
  println!("Display: {list}");
}