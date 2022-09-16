Borrowing and Dereferencing Operators

Types #

Borrowing can be of two types:

    Shared borrowing

    A piece of data that is shared by single or multiple variables but it cannot be altered

    Mutable borrowing

    A piece of data that is shared and altered by a single variable 
    (but the data is inaccessible to other variables at that time)

fn main() {
    let x = 10;
    let mut y = 13;
    //immutable reference to a variable
    let a = &x;
    println!("Value of a:{}", a); 
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed
    //mutable reference to a variable
    let b = &mut y;
    println!("Value of b:{}", b);

    *b = 11; // derefencing 
    println!("Value of b:{}", b); // updated value of b
    println!("Value of y:{}", y); // y value can be changed as it is mutuably borrowed
}

Output
0.88s

Value of a:10
Value of x:10
Value of b:13
Value of b:11
Value of y:11

Dereferencing Operator


fn main() {
    //mutable reference to a variable
    let mut x = 10;
    println!("Value of x:{}", x);
    let a = & mut x;
    println!("Value of a:{}", a);
    //dereference a variable
    *a = 11;
    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // Note that value of x is updated
}

Output
0.91s

Value of x:10
Value of a:10
Value of a:11
Value of x:1