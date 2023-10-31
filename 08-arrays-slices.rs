// https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html

use std::mem;

// this functoin borrows a slice - we don't know the size
fn analyze_slice(slice: &[i32]) {
  println!("First element of the slice: {}", slice[0]);
  println!("The slice has {} elements", slice.len());
}

fn main() {
  let x: [i32; 5] = [1, 2, 3, 4, 5];

  println!("{x:?}");
  // array indexing is [] instead . as in tuples
  println!("{}", x[0]);

  // all can be initialized to same value via `;`
  let y: [i32; 500] = [0; 500];

  println!("{y:?}");
  println!("{}", y.len());

  println!("Array occupies {} bytes.", mem::size_of_val(&y));

  // borrow slice
  analyze_slice(&x);
  analyze_slice(&y[20..300]);
  analyze_slice(&y[20..=300]);

  // empty slice
  let empty: [u32; 0] = [];
  assert_eq!(&empty, &[]);
  assert_eq!(&empty, &[][..]); // same - probably expanding all elements???

  for i in 0..x.len() + 1 {
    match x.get(i) { // safe acces even when out of range
      Some(val) => println!("{i}: {val}"),
      None => println!("Out of range index: {i}"),
    }
  }
}
