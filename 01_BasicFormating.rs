//-------------------------------------Single Place Holder----------------

fn main() {
    println!("Number: {}", 1);
}
// Output:
// Number: 1

//------------------------------------Multiple Place Holder--------------

fn main() {
    println!("{} is a {} company", "Educative", "Software");
}

// Output:
// Educative is a Software company

//----------------------------------- Positional Arguments----------------

fn main() {
    println!(
        "Enhance your coding skills from {0} courses.  {0} courses are very {1}",
        "Educative", "interactive"
    );
}

// Output:
// Enhance your coding skills from Educative courses.  Educative courses are very interactive

//------------------------------------------ Named Arguments----------------
fn main() {
    println!(
        "{company} provides {kind} courses",
        company = "Educative",
        kind = "interactive"
    );
}

// Output:
// Educative provides interactive courses

//------------------------------------------ Placeholder Traits----------------

// If we want to convert the value to binary, hexadecimal, or octal write:
// {:b},{:x},{:o}

fn main() {
    println!(
        "Number : 10 \n Binary:{:b} \n Hexadecimal:{:x} \n Octal:{:o}",
        10, 10, 10
    );
}

// Output:
// Number : 10
// Binary:1010
// Hexadecimal:a
// Octal:12

//--------------------------------------------- Basic Math----------------

fn main() {
    println!("{} + {} = {}", 10, 10, 10 + 10);
}
// Output:
// 10 + 10 = 20

//-------------------------------------------- Placeholder for a Debug Trait -----------

// You can use a debug trait and write as many values as desired within the parentheses.
fn main() {
    println!("{:?}", ("This is a Rust Course", 101));
}

// Output:
// ("This is a Rust Course", 101)
