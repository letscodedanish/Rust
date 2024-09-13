# Day 1 - Rust Learning Notes

## 1. Structs

### Concept:
A `struct` in Rust allows me to group related data together. It’s like creating a custom data type that can hold multiple values. I can define a `struct` with fields and access them later.

### Code:
```rust
struct User {
  active: bool,
  age: i32,
  username: String,
}

struct Rect {
  width: u32,
  height: u32,
}
Here, I defined two structs: User and Rect. The User struct holds a boolean, an integer, and a string, while the Rect struct holds two unsigned integers for width and height.

2. Methods on Structs
Concept:
I can define methods on a struct using the impl block. Methods allow me to associate behavior with the struct and perform operations on its fields.

Code:
rust
Copy code
impl Rect {
  fn area(&self) -> u32 {
    return self.width * self.height;
  }

  fn peri(&self, num: u32) -> u32 {
    println!("Printed statement: {}", (self.width + self.height) * num);
    return 2 * (self.width + self.height);
  }

  fn debug() -> i32 {
    return 1;
  }
}
In the impl block for the Rect struct:

area() calculates the area of the rectangle.
peri() calculates the perimeter and prints a statement.
debug() simply returns an integer for testing.
3. Enums
Concept:
Enums in Rust allow me to define types by listing their possible variants. Each variant can hold different types of data, making them powerful for handling multiple possibilities under one type.

Code:
rust
Copy code
enum Shapes {
  Rectangle(f64, f64),
  Circle(f64),
}
Here, I defined an enum Shapes with two variants:

Rectangle with width and height.
Circle with a radius.
4. Pattern Matching with match
Concept:
The match statement in Rust allows me to handle different enum variants with pattern matching. This makes it easy to work with enums and other types by matching on their values.

Code:
rust
Copy code
fn cal_area(shape: Shapes) -> f64 {
  let ans = match shape {
    Shapes::Rectangle(width, height) => width * height,
    Shapes::Circle(radius) => 3.14 * radius * radius,
  };
  return ans;
}
In this example, I used match to calculate the area of either a rectangle or a circle based on which variant of the Shapes enum is passed.

5. Option Type
Concept:
Rust’s Option type represents a value that may or may not be present. It’s used instead of null, providing a way to handle the absence of a value safely.

Code:
rust
Copy code
fn find_first(s: String) -> Option<i32> {
  for (index, char) in s.chars().enumerate() {
    if char == 'a' {
      return Some(index as i32);
    }
  }
  return None;
}

fn main() {
  let index = find_first(String::from("preet"));
  match index {
    Some(val) => println!("Index found at {}", val),
    None => println!("Index Not found"),
  }
}
In this example:

find_first() searches for the first occurrence of the character 'a' in a string. If it finds it, it returns Some(index); otherwise, it returns None.
In main(), I used match to handle both cases: whether the index was found or not.
6. Iteration and Enumeration
Concept:
Using the .chars().enumerate() method, I can iterate over a string, getting both the index and the character at the same time. This is useful for operations like searching through a string.

Code:
rust
Copy code
for (index, char) in s.chars().enumerate() {
  if char == 'a' {
    return Some(index as i32);
  }
}
In this loop, I get both the index and the character, allowing me to return the position of the first 'a' in the string.

7. Ownership and Borrowing
Concept:
In Rust, I need to be aware of how data ownership works. When passing a value like a String into a function, I pass ownership of that value unless I explicitly borrow it using references. Rust's ownership system helps me avoid common bugs like dangling pointers.

Code:
rust
Copy code
let index = find_first(String::from("preet"));
Here, I’m passing ownership of the string to the find_first function, and I no longer have access to the original string after the function call.
