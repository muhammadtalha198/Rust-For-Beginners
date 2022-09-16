//========================= Types of Comments in Rust ======================//

Line Comments

// Writing a Rust program
fn main() {
    //The line comment is the recommended comment style
    println!("This is a line comment!"); // print hello World to the screen
}

Block Comments

fn main() {
    /* 
    The block comment is extremely useful for temporarily disabling
    a large chunk of code. /* Block comments can also be /* nested, */ */
    To comment a large block just write in between /* text */
    */
    println!("This is a line comment!"); // print hello World to the screen
}

//========================= Doc Comments ======================//

// The comments followed by /// or //! are known as doc comments. Let’s see the difference between the two sets of doc comments.

// ==========Outer Doc Comments /// ===========//

// Outer doc comments are written outside the block of code.

//============Inner Doc Comments //! =========//

//Inner doc comments are written inside the block of code.

/// This is a Doc comment outside the function
/// Generate docs for the following item.
/// This shows my code outside a module or a function
fn main() {
    //! This a doc comment that is inside the function   
    //! This comment shows my code inside a module or a function  
    //! Generate docs for the enclosing item
    println!("{} can support {} notation","Doc comment","markdown");
}