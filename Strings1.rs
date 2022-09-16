fn main() {
  //define a primitive String variable
  let language:&str = "Rust";
  //print the String literal
  println!("String literal: {}", language);
  //print the length of the String literal
  println!("Length of the string literal: {}", language.len());
}

Output
0.71s

String literal: Rust
Length of the string literal: 4


// String Object (String)

fn main() {
  // create an empty String
  let course1 = String::new();
  // convert String literal to String object using .to_string
  let s_course1 = course1.to_string();
  // print the String object
  println!("This is an empty string {}.", s_course1);
  // print the length of an empty String  object
  println!("This is a length of my empty string {}.", s_course1.len());

  // create a String literal
  let course2 = "Rust Programming";
  // convert String literal to string object using .to_string
  let s_course2 = course2.to_string();
  // print the String object
  println!("This is a string literal : {}.", s_course2);
  // print the length of a String object
  println!("This is a length of my string literal {}.", s_course2.len());

  // define a String object using from method
  let course3 = String::from("Rust Language");
  // print the String object
  println!("This is a string object : {}.", course3);
  // print the length of an string object
  println!("This is the length of my string object {}.", course3.len());
}


Output
3.53s

This is an empty string .
This is a length of my empty string 0.
This is a string literal : Rust Programming.
This is a length of my string literal 16.
This is a string object : Rust Language.
This is the length of my string object 13.

////////////////Capacity in Bytes//////////////////

fn main() {
  // define a growable string variable
  let course = String::from("Rust");
  println!("This is a beginner course in {}.", course);
  //capacity in bytes
  println!("Capacity: {}.", course.capacity());
}

Output
1.13s

This is a beginner course in Rust.
Capacity: 4.
