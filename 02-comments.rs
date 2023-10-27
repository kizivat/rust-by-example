// comments are pretty normal

/*
  they don't recommend block comments
  but are useful for commenting out code
*/

/// this are used for documentation of the "following item"
fn main () {
  //! and this is supposed to be used for documentation of the "enclosing item" (what it means??)
  let x = 5 + /* 90 + */ 5;
  println!("Here, `x` would be {}, obviously", x);
}