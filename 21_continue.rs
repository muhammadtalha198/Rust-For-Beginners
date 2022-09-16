

Continue Statement



fn main() {
  // define a for loop
  for var in 0..10 {
     if var == 4 {
        println!("I encoutered a continue statement");
        continue;
      }
      println!("var: {}", var);
      println!("I did not encounter continue statement");
  }
}

Output
1.5s

var: 0
I did not encounter continue statement
var: 1
I did not encounter continue statement
var: 2
I did not encounter continue statement
var: 3
I did not encounter continue statement
I encoutered a continue statement
var: 5
I did not encounter continue statement
var: 6
I did not encounter continue statement
var: 7
I did not encounter continue statement
var: 8
I did not encounter continue statement
var: 9
I did not encounter continue statement


Using With a while Loop#

fn main() {
    // define an integer variable
    let mut var = 1; 
    // define a boolean variable
    let mut found = false;
    // define a while loop
    while !found {
      var = var + 1;
      println!("{}", var);
      
      if var == 4 {
          println!("I encoutered a continue statement");
          continue;
        }
        println!("I did not encounter continue statement");
        
        if var == 10{
          found = true;
        }
    }
}

Output
0.67s

2
I did not encounter continue statement
3
I did not encounter continue statement
4
I encoutered a continue statement
5
I did not encounter continue statement
6
I did not encounter continue statement
7
I did not encounter continue statement
8
I did not encounter continue statement
9
I did not encounter continue statement
10
I did not encounter continue statement
