use std::fmt;
    use std::fmt::Formatter;


fn main() {
  let string = format!("This writes formatted text to a String, {}", "like this");
  println!("I can now put it here: {}", string);

  println!("The number {} is stringified", 33);

  // You can also use named arguments
  println!("{hello} {world}", hello="Don't hello,", world="nobody");

  // here are different ways to format numbers
  println!("{} {:b} {:o} {:x} {:X}", 15, 15, 15, 15, 15);

  // justify
  println!("{number:>6}", number=1); // will allign number to the right with a width of 6
  println!("{number:>width$}", number=12, width=6); // by appending $ to the variable name, I can use a variable for the width
  println!("{number:>width$}", number=123456, width=6);
  println!("{number:>width$}", number=1234567, width=6); // this will simply overflow, no truncation

  // since 1.58 this will capture the "surrounding" variables
  let num = 89;
  let len = 6;
  println!("{num:>len$}");

  // Rust even checks to make sure the correct number of arguments are used.
  println!("My name is {0}, {1} {0}", "Bond", "James");
  // done
  // FIXME ^ Add the missing argument: "James"

  // i32 represents a signed 32-bit integer, but not sure what it means 
  // with respect to the structure - is it constructor arguments?
  #[allow(dead_code)]
  #[derive(Debug)]
  struct Structure(i32);

  // bellow won't work as `Structure` doesn't implement `std::fmt::Display`
  println!("This struct `{:?}` wouldn't print with braces...", Structure(3));

  struct Structure2(i32);

  impl fmt::Display for Structure2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
      write!(f, "{}", self.0)
    }
  }

  println!("This struct `{}` should print with braces...", Structure2(4));

  let pi = 3.141592;
  let prec = 3;
  println!("Pi is roughly {pi:.prec$}", )
}
