Types of Conditional Expression 

There are three types of conditional expression in Rust:

1. if Expression

2. if let Expression

3. match Expression

fn main() {
      //define a variable 
      let learn_language="Rust";
      // if..elseif..else construct 
      if learn_language == "Rust" { 
         println!("You are learning Rust language!");
      }
      else if learn_language == "Java" { 
         println!("You are learning Java language!");
      }
      else {
         println!("You are learning some other language!");
      } 
}

Output
0.72s

You are learning Rust language!


Shorthand if 

fn main() {
    //define a variable  
    let learn_language = "Rust";
    // short hand construct
    let res= if learn_language == "Rust" {"You are learning Rust language!"} else {"You are learning some other language!"};
    println!("{}", res);
}

Output
0.69s

You are learning Rust language!

fn main() {
    let x = "Rust";

    let y: bool = if x == "Rust" { true } else { false };

    // let z: bool = if x == "Rust" { true; } else { false; };

    println!("x:{}", x);
    println!("y:{}", y);

}

Output
0.69s

x:Rust
y:true
