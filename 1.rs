// // struct User{
// //   active: bool,
// //   age: i32,
// //   username: String,
// // }

// // struct Rect{
// //   width: u32,
// //   height: u32,
// // }

// // impl Rect{
// //   fn area(&self) -> u32{
// //      return self.width * self.height;
// //   }

// //   fn peri(&self, num: u32) -> u32{
// //     println! ("printed statement: {}",(self.width + self.height) * num);
// //     return 2* (self.width + self.height);
// //   }

// //   fn debug() -> i32 {
// //     return 1;
// //   }
// // }

// enum Shapes {
//   Rectangle(f64, f64),
//   Circle(f64),
// }

// fn main(){
//   // let user1 = User{
//   //   active: true,
//   //   age: 32,
//   //   username: String::from("Danish"),
//   // };
//   // println!("{}", user1.username);
//   // println!("{}", user1.age);
//   // println!("{}", user1.active);

//   // let rect1 = Rect{
//   //   width: 30,
//   //   height: 50,
//   // };
//   //   println!("the area of the rectangle is: {}", rect1.area());
//   // println!("the perimeter of the rectangle is: {}", rect1.peri(2));
//   // println!("the debug of the rectangle is: {}", Rect::debug());

//   let rect = Shapes::Rectangle(3.3, 4.3);
//   cal_area(rect);
//   let circle = Shapes::Circle(4.3);
//   cal_area(circle);

// }

// fn cal_area(shape: Shapes) -> f64 {
//   let ans = match shape{
//       Shapes::Rectangle(width, height) => width * height,
//       Shapes::Circle(radius) => 3.14 * radius * radius,
//   };
//   return ans;
// }

fn main(){
  let index =  find_first(String::from("preet"));

  match index {
    Some(val) => println!("Index found at {}", val),
    None => println!("Index Not found"),
  }
  // if index == -1{
  //   println!("Index Not found");
  // }
  // else{
  //   println!("Index found at {}", index);  
  // }
}

fn find_first(s: String) -> Option<i32> {
  for(index,char) in s.chars().enumerate(){
    if char == 'a'{
      return Some(index as i32);
    }
  }
   return None;
}
