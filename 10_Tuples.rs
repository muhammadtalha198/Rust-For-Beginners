Tuples
fn main() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // access value of a tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);

    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // get individual values out of tuple
    let (w ,x, y, z) = person_data;
    //print values
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Weight : {}", y);
    println!("Height : {}", z);
}

Output
0.74s

The value of the tuple at index 0 and index 1 are Alex 48
Name : Alex
Age : 48
Weight : 35kg
Height : 6ft


Print the Tuple

fn main() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);
}

Output
0.81s

Tuple - Person Data : ("Alex", 48, "35kg", "6ft")

fn test() {
    // define a tuple
    let persons = ("Alex",21, "Abe", 22, "Anna", 23);
    // print the values of tuple
    print!("{}:{}, {}:{}, {}:{}", persons.0, persons.1, persons.2, persons.3, persons.4, persons.5);
}

Output
0.82s

Alex:21, Abe:22, Anna:23