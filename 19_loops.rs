Loop - For Loop

fn main() {
    //define a for loop 
    for i in 0..5 {
      println!("{}", i);
    }
}


Output
0.71s

0
1
2
3
4


Enumerat
To count how many times the loop has already executed, use the .enumerate() function.

fn main() {
  for (count, variable) in (7..10).enumerate() {
      println!("count = {}, variable = {}", count, variable);
  }
}


Output
1.15s

count = 0, variable = 7
count = 1, variable = 8
count = 2, variable = 9

While loop#


fn main() {
  let mut var = 1; //define an integer variable
  let mut found = false; // define a boolean variable
  // define a while loop
  while !found {
      var=var+1;
      //print the variable
      println!("{}", var);
      // if the modulus of variable is 1 then found is equal to true
      if var % 2 == 1 {
        found = true; 
      }
      println!("Loop runs");
  }
}

Output
1.09s

2
Loop runs
3
Loop runs


Loop Labels

fn main() {
 'outer:for i in 1..5 { //outer loop
    println!("Muliplication Table : {}", i);
   'inner:for j in 1..5 { // inner loop
        if i == 3 { continue 'outer; } // Continues the loop over `i`.
        if j == 2 { continue 'inner; } // Continues the loop over `j`.
        println!("{} * {} = {}", i, j, i * j);
   }
 }
}

Output
0.7s

Muliplication Table : 1
1 * 1 = 1
1 * 3 = 3
1 * 4 = 4
Muliplication Table : 2
2 * 1 = 2
2 * 3 = 6
2 * 4 = 8
Muliplication Table : 3
Muliplication Table : 4
4 * 1 = 4
4 * 3 = 12
4 * 4 = 16
