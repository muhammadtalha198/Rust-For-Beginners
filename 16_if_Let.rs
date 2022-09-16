If Let Expression

fn main() {
    // define a scrutinee expression    
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner","course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }
}

Output
0.77s

Wrote all values in pattern to be matched with the scrutinee expression
fn main() {
    // define a scrutinee expression    
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", c) = course {
        println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    } 
    else {
        // do not execute this block
        println!("Value unmatched");
    }
}

Output
0.71s

Wrote first two values in pattern to be matched with the scrutinee expression : course


fn main() {
    // define a scrutinee expression     
    let course = ("Rust", "beginner");
    // pattern does not match with the scrutinee expression
    if let ("Java", c) = course {
        println!("Course is {}", c);
    } else {
        // execute this block
        println!("Value unmatched");
    }
}

Output
0.76s

Value unmatched
 