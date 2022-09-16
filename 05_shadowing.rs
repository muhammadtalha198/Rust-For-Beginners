// =============== shadowing In Rust=================

fn shadowing() {
    let x = 24;
    println!("The value of y will be: {}", x);

    let x = 45; // shadowing => having same name of a variable
    println!("The value of y will be: {}", x);

    // x = 34; not allowed gives an error
}

fn shadowing_2() {
    let outer_variable = 112;
    {
        let inner_variable = 213;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117;
        println!("block variable outer: {}", outer_variable);
    }
    println!("outer variable: {}", outer_variable);
}