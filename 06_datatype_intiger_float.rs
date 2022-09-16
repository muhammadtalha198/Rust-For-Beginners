// =============== datatypes In Rust=================
Integers and Floats

fn main() {
    //explicitly define an integer
    let a:i32 = 24;
    let b:u64 = 23;
    let c:usize = 26;
    let d:isize = 29;
    //print the values
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    
}

Output
1.84s

a: 24
b: 23
c: 26
d: 29


Floating Point
fn main() {
    //explicitly define a float type
    let f1:f32 = 32.9;
    let f2:f64 = 6789.89;
    println!("f1: {}", f1);
    println!("f2: {}", f2);
}

Output
1.88s

f1: 32.9
f2: 6789.89
