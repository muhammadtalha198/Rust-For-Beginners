//============= Bool in Rust ==============

fn main() {
    //explicitly define a bool
    let is_bool: bool = true;
    println!("explicitly_defined: {}", is_bool);
}
// Output:
// => explicitly_defined: true

fn main() {
    // get a value from an expression
    let c = 10 > 2;
    println!("c: {}", c);
}

// Output
// => c: true
