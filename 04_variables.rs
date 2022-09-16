//============= Naming convention in Rust ==============

// CamelCase => myNameIsKhan.
// SnakeCase => my_name_is_khan.

// =============== Variables and Constants In Rust=================

fn main() {
    let num = 12;
    let myName = "Muhammad";

    // every value of variable will be print in the place holder({}).
    println!("the value of num will be: {}", num);
    println!("the value of myName will be: {}", myName);

    mutuable(); // function call
    immutable(); //function call but will create an error. un=comment and check!

    variable_example();
    constants_in_rust();
    shadowing();
}

fn variable_example() {
    let x: i32 = 1;
    println!("The value of x will be: {}", x);

    let y: bool = true;
    println!("The value of y will be: {}", y);
}

// =============== Mutuable variable In Rust=================

fn mutuable() {
    //To make variable change=able we use the mut=keyword
    let mut y = 10;
    println!("The value of y will be: {}", y);

    y = 60;
    println!("The value of y will be: {}", y);
}

fn mutuable_2() {
    let mut language = "Rust"; // define a mutable variable
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable
}

// =============== ImMutuable variable In Rust=================

fn immutable() {
    let x = 45;

    println!("the value of x will be: {}", x);

    // x = 60; ==============This will be an error
    //===============You can not re=assign a inmutuable variable
    //===============Variable are immuteable by=default

    println!("The value of x will be: {}", x);
}


// =============== Assigning Multiple Variables In Rust=================

fn assigning_multiple_variables() {
    let (course, category) = ("Rust", "beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
}
