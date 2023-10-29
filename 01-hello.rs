fn main() {
  // the ! means it's a macro... whatever that means
    // ok so macro is basically just compile-time expanded code,
    // but somehow can also execute code at compile-time - not sure how that works for now
  println!("Hello, world!"); // we do need semicolons at the end of lines
  println!("I'm a Rustacean!")
  // looks like we don't need semicolons before closing braces
    // it is not that we do not need semicolons before closing braces
    // but rather that the return value of the last expression is what
    // the surrounding expresion resolves into, if there is no semicolon
    // besically we don't need `return` keyword for the function to resolve into
    // a value - if we don't add the semicolon to the last expression, the
    // function resolves to the same value as the last expression.
    // the semicolon seems to "swallow" the value if added
}