Finding a Substring

Syntax#

The general syntax is :

str.contains("sub_str")


fn main() {
  // define a growable string variable
  let str = String::from("Rust Programming"); 
  let sub_str = String::from("Rust"); 
  println!("This is a beginner course in {}.", str);
  // find if string contains a substring
  println!("{} is a substring of {}: {}.", sub_str, str, str.contains("Rust"));
}

Output
1.35s

This is a beginner course in Rust Programming.
Rust is a substring of Rust Programming: true.


Replace a Substring

The general syntax is :

str.replace(replace_from, replace_to)

fn main() {
  // define a growable string variable
  let str = String::from("Rust Programming"); 
  let replace_from = "Programming";
  let replace_to = "Language"; 
  // find if string contains a substring
  let result = str.replace(replace_from, replace_to);
  println!("{} now becomes {}.", str, result);
}

Output
0.78s

Rust Programming now becomes Rust Language.




Trim a String


// To trim a string use the function trim(). 
// It is used to remove leading and trailing whitespaces in a string.
// The general syntax is :

// string.trim()


fn main() {
   let string = "   Rust     Programming     ".to_string();
   let trim_string = string.trim(); 
   // get characters at 5,6,7,8,9,10 and 11 indexes
   println!("Trimmed_string : {}", trim_string);
}

Output
1.78s

Trimmed_string : Rust     Programming
