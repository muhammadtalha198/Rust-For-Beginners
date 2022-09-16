Type Casting in Rust

fn main() {
    let a = 15;
    let b = (a as f64) / 2.0; 
    println!("a: {}", a);
    println!("b: {}", b);
}

Output
0.77s

a: 15
b: 7.5



    📝What data types can be type casted?

        Integer can be type casted to floating-point and vice versa.
        Integer can be typecasted to String

    📝What data types cannot be type casted?

        String (&str) or character cannot be type casted to the data type of type integer or float.
        Character cannot be type casted to String type and vice versa.

