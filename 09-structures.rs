#![allow(dead_code)]

use std::fmt;

struct NamedTuple(f64, i32);

// C like struct
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "name: {}, age: {}", self.name, self.age)
  }
}

// allegedly useful for generics
#[derive(Debug)]
struct UnitStruct;

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

#[derive(Debug)]
struct Rect {
  start: Point2D,
  end: Point2D,
}

fn rect_area(rect: Rect) -> f64 {
  let Rect {
    start: Point2D { x: start_x, y: start_y },
    end: Point2D { x: end_x, y: end_y }
  } = rect;
  ((start_x - end_x) * (start_y - end_y)).abs()
}

fn square(start: Point2D, a: f64) -> Rect {
  Rect {
    start: Point2D { ..start },
    end: Point2D { x: start.x + a, y: start.y + a }
  }
}

fn main() {
  // still don't understand how strings work... why do I need String::from??
  let name = String::from("David");
  let age = 30u8;

  let me = Person { name, age };
  println!("{me:?}");
  println!("{me}");

  let point = Point2D { x: 3.14, y: 32.1 };
  println!("Coords: {} {}", point.x, point.y);

  // struct update syntax
  let point2 = Point2D { x: 4.2, ..point }; // i would expect this to override x as well as spread in JS but it doesn't
  println!("Second: {} {}", point2.x, point2.y);

  // base struct must always be the last field
  // let point22 = Point2D { ..point, x: 4.2 };
  // println!("Second: {} {}", point22.x, point22.y);

  let point3 = Point2D { ..point2 };
  println!("Third: {} {}", point3.x, point3.y);

  // destructuring:
  let Point2D { x: named_x, y } = point3;
  println!("x: {named_x}, y: {y}");

  let rect = Rect {
    start: point,
    end: Point2D { x: 101.2, y:  1.3 } 
  };
  println!("{rect:?}");
  println!("Area: {}", rect_area(rect));

  let unit = UnitStruct;
  println!("{unit:?}");

  print!("{:?}", square(point2, 2.2))
}
