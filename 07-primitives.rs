use std::fmt;


// for Matrix activity at https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "({} {})", self.0, self.1)?;
    write!(f, "({} {})", self.2, self.3)
  }
}

fn main() {
  // suffix type for numbers
  println!("{}", 2i32);

  // experiment: will it overflow?
    // it panics!
  // let a = 2u32;
  // let b = 3u32;
  // println!("{} - {} = {}", a, b, a - b);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);
  println!("{}", matrix);
}