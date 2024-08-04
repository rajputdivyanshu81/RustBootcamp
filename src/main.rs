// use std::char;

// fn main() {

//     // this is to understadn the basic concept of the rust


//     println!("Hello, world!");
//     let mut x: i32 = 5;

//     print!("x:{}", x);
//     print!("\n"); // Add a semicolon here

//     for i_ in 0..1000 {
//         x = x + 100;
//     }

//     print!("x = {}", x);
    
//     print!("\n"); // Add a semicolon here


//     // loop conditionals in rust 

//      let is_male = true ;
//      let is_above_18 = true;

//      if is_male {
//         print!("you are a male\n");
//      }
//      else{
//         print!("you are not a male");
//      }
//     print!("\n"); // Add a semicolon here

//      if is_male && is_above_18 {
//         print!("you are a legal male");
//      }

//      //this is the way to initialize the string in the rust
     

//     print!("\n"); // Add a semicolon here
     
//      let greeting = String::from("hello world");
//      print!("{}",greeting);

//     print!("\n"); // Add a semicolon here

//      let char1: Option<char> = greeting.chars().nth(1000);


//      match char1 {
//          Some(c) => println!("{}", c),
//          None => println!("No character at the index 1000"),
//      }


//      //loops in rust and conditionals

//      let is_even : bool = true;

//      if is_even {
//       println!("the number is even");
//      }
//      else if !is_even {
//       println!("the number is odd");
//      }


//      //function
//      let a:i32 = 10;
//      let b: i32 = 20;

//      let sum :i32  = do_sum(a, b);
//      println!("sum of {} and {} is {}",a,b,sum);

//      //loops in rust

//      // agar 10 tak print krna hai to 11 tak lena hoga loop


//       for i  in  0..11  {
//          println!("{}",i);   
//       }
//       pub fn main() {
//          let str = String::from("harkirat singh");
//          println!("First name: {}", get_first_name(str));
//      }
     
//      pub fn get_first_name(str: String) -> String {
//          let mut first_name = String::from("");
//          for c in str.chars() {
//              if c == ' ' {
//                  break;
//              }
//              first_name.push(c);
//          }
//          first_name
//      }
     
     

     

//      // stack heap and the concept of string 

//       stack_fn();   // Call the function that uses stack memory
//       heap_fn();    // Call the function that uses heap memory
//       update_string();  // Call the function that changes size of variable at runtime
//   }
  
//   fn stack_fn() {
//       // Declare a few integers on the stack
//       let a = 10;
//       let b = 20;
//       let c = a + b;
//       println!("Stack function: The sum of {} and {} is {}", a, b, c);
//   }
  
//   fn heap_fn() {
//       // Create a string, which is allocated on the heap
//       let s1 = String::from("Hello");
//       let s2 = String::from("World");
//       let combined = format!("{} {}", s1, s2);
//       println!("Heap function: Combined string is '{}'", combined);
//   }
  
//   fn update_string() {
//       // Start with a base string on the heap
//       let mut s = String::from("Initial string");
//       println!("Before update: {}", s);
//       println!("capacity : {} ,Length : {},pointer : {:p}",s.capacity(),s.len(),s.as_ptr());
  
//       // Append some text to the string
//       s.push_str(" and some additional text");
//       println!("capacity : {} ,Length : {},pointer : {:p}",s.capacity(),s.len(),s.as_ptr());

//       println!("After update: {}", s);


//       //mutability


//       //in numbers

//       let mut x: i32 = 1;
//       x = 2;
//       println!("{}",x);


//       //in strings

//       let mut x: String = String::from("hii there");
//       x.push_str("nvmsndc");
//       println!("{}", x);
      

//       // ownership and how it gets to know that it us the right time to dealocate the memory

//       let s1 = String::from("hi there");
//       let s2 = s1;
//       //we can not use s1 here because it will die
//       println!("{}",s2);
//       //here if we assign a value to the another variable then it woll move to the another person and moved on from the previous one like rihana which is harkirat's girlfriend
      

      
//          let my_string = String::from("Hello, Rust!");
//          takes_ownership(&my_string);  // Pass a reference to my_string
//          println!("{}", my_string);    // This is valid because ownership was not transferred
//      }
     
//      fn takes_ownership(some_string: &String) {
//          println!("{}", some_string);  // some_string is borrowed and not moved
     

//      // and after moving on rihana can came back to from her present boyfriend with harkirat
//       let s1 = String::from("Hello");
//       let s2 = &s1;
  
//       println!("{}", s2);
//       println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
      
//       let mut s1:String = String::from("hello");
//       s1.push_str("world");
//       println!("{}",s1);

  
//  }
 
//  fn do_sum(a: i32, b: i32) -> i32 {
//      return a + b;
//  }
 
//  // to update any variable firstly u needs have a mut variable so that we can update as whenever we are needs to update


//     // the whole rust is compleltly based on rihana



// struct User {
//    name : String,
//    age : u32,
//    active:bool,
// }
// fn main(){
//    let name:String = String::from("Alice");
//    let user: User = User {
//       name: name,
//       age:30,
//       active:true,
//    };
//    println!("{} is {} years old",user.name,user.age)
// }

// struct Rect {
//    width: u32,
//    height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//          self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     print!("The area of the rectangle is {}", rect.area());
// }




// #[derive(Debug)]
// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }

// fn main() {
//     let my_direction = Direction::North;
//     let new_direction = my_direction; // No error, because Direction is Copy
//     move_around(new_direction);
// }

// fn move_around(direction: Direction) {
//     println!("{:?}", direction); // Print the direction for debugging
//     // Implement logic to move a character around
// }


// Define an enum called Shape
// enum Shape {
//    Circle(f64),  // Variant with associated data (radius)
//    Square(f64),  // Variant with associated data (side length)
//    Rectangle(f64, f64),  // Variant with associated data (width, height)
// }

// // Function to calculate area based on the shape
// fn calculate_area(shape: Shape) -> f64 {
//    match shape {
//        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//        Shape::Square(side) => side * side,
//        Shape::Rectangle(width, height) => width * height,
//    }
// }

// fn main() {
//    // Create instances of different shapes
//    let circle = Shape::Circle(5.0);
//    let square = Shape::Square(4.0);
//    let rectangle = Shape::Rectangle(3.0, 6.0);

//    // Calculate and print the area of each shape
//    println!("Area of the circle: {}", calculate_area(circle));
//    println!("Area of the square: {}", calculate_area(square));
//    println!("Area of the rectangle: {}", calculate_area(rectangle));
// }

// struct  Point<T> {
//    x: T,
//    y:T,
// }

// fn main(){
//    let integer_point: Point<i32> = Point{x:5,y:10};
//    let float_point : Point<f64> =  Point{x:1.0,y:4.2};
//    println!("Integer Point :({},{})",integer_point.x,integer_point.y);
//    println!("float Point ({},{})",float_point.x,float_point.y);
// }


// use std::{fs, io::Error};

// fn main() {
//     let res: Result<String, Error> = fs::read_to_string("example.txt");

//     match res {
//         Ok(content) => println!("File content: {}", content),
//         Err(err) => println!("Error: {}", err),
//     }
// }


//opton enum

// fn find_first_a(s: String) -> Option<i32> {
//    for (index, character) in s.chars().enumerate() {
//        if character == 'a' {
//            return Some(index as i32);
//        }
//    }
//    return None;
// }

// fn main() {
//    let my_string = String::from("raman");
//    match find_first_a(my_string) {
//        Some(index) => println!("The letter 'a' is found at index: {}", index),
//        None => println!("The letter 'a' is not found in the string."),
//    }
// }

// use chrono::prelude::*;


// fn main() {
//     // Get the current date and time
//     let now: DateTime<Utc> = Utc::now();
//     println!("Current date and time: {}", now);

//     // Format the date and time
//     let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
//     println!("Formatted date and time: {}", formatted);
// }



