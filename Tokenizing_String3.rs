Tokenizing to Separate on Whitespaces

The general syntax is:
for found in  str.split_whitespace(){
    println!("{}", found);
}


fn main() {
  // define a String object
  let str = String::from("Rust Programming"); 
  // split on whitespace
  for token in str.split_whitespace(){
      println!("{}", token);
  }
}

Output
0.88s

Rust
Programming


Tokenizing to Split on a Custom Character

The general syntax is:

for found in str.split(","){
    println!("{}", found);
}

fn main() {
  // define a String object
  let str = String::from("Educative, course on, Rust, Programming");  
  // split on token
  for token in str.split(","){
      println!("{}", token);
  }
}

Educative
 course on
 Rust
 Programming


 Iterating Over the String Object


fn main() {
  // define a String object
  let str = String::from("Rust Programming");  
  // split on literal
  for token in str.chars(){
      println!("{}", token);
  }
}

 Output
0.72s

R
u
s
t
 
P
r
o
g
r