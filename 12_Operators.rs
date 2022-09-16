fn main() {
    let a = 4;
    let b = 3;
    
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("Addition:{}", a + b);
    println!("Subtraction:{}", a - b);
    println!("Multiplication:{}", a * b);
    println!("Division:{}", a / b);
    println!("Modulus:{}", a % b);
}

Output
0.69s

Operand 1:4, Operand 2:3
Addition:7
Subtraction:1
Multiplication:12
Division:1
Modulus:1

Logical Operators

fn main() {
  let a = true;
  let b = false;
  println!("Operand 1:{}, Operand 2:{}", a , b);
  println!("AND:{}", a && b);
  println!("OR:{}", a || b);
  println!("NOT:{}", ! a);
}

Output
1.14s

Operand 1:true, Operand 2:false
AND:false
OR:true
NOT:false


Comparison Operators


fn main() {
    let a = 2;
    let b = 3;
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("a > b:{}", a > b);
    println!("a < b:{}", a < b);
    println!("a >= b:{}", a >= b);
    println!("a <= b:{}", a <= b);
    println!("a == b:{}", a == b);
    println!("a != b:{}", a != b);
}

Output
1.42s

Operand 1:2, Operand 2:3
a > b:false
a < b:true
a >= b:false
a <= b:true
a == b:false
a != b:true


Assignment and Compound Assignment Operators

fn main() {
   let a = 2;
   let b = a;
   println!("b = a");
   println!("Value of a:{}", a);
   println!("Value of b:{}", b);
}

Output
0.8s

b = a
Value of a:2
Value of b:2

Compound Assignment


fn main() {
    //define a mutable variable
    let mut a = 2;
    println!("a:{}", a);
    a += 1;
    println!("a+=1:{}", a);
    println!("a:{}", a);
    a -= 1;
    println!("a-=1:{}", a);
    println!("a:{}", a);
    a /= 1;
    println!("a/=1:{}", a);
    println!("a:{}", a);
    a *= 3;
    println!("a*=3:{}", a);
}

Output
0.9s

a:2
a+=1:3
a:3
a-=1:2
a:2
a/=1:2
a:2
a*=3:6

