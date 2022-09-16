//========================Chalange 1====================//

// => You have to print the following statement to the console:
// -----"I am learning Rust programming language"----

// Solution:

fn test() {
    print!("I am learning Rust programming language");
}



//========================Chalange 2===================//

//  => The task is to display the pyramid of numbers in the following format.
// ----- 1
// ----- 22
// ----- 333
// ----- 4444
// ----- 55555

// Solution:

fn test() {
    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}", 5, 5, 5, 5, 5);
}