//=================Declare a Struct==================

struct StructName {

    item1: datatype,
    item2: datatype,
    item3: datatype,

    itemN: datatype,    
}

// ==> The name of the struct should be in PascalCase, 

//================ Initialize a Struct================

let my_name = StructName{

    item1: value1,
    item2: value2,

    itemN: valueN,
}

// =====================Access Values from a Struct=======

StructName.item_name;


//=====================Update a Struct Instance============

let mut my_name = StructName{
    
    item1: value1, 
    item2: value2,
    
    itemN: valueN,
}

my_name.item1 = my_new_value;


//=================Example======================

// declare a Struct
struct Course{
    code: i32,
    name: String,
    level: String,
}

fn main(){
    
    //initialize
    let mut course1 = Course{
        code: 130,
        level: String::from("beginner"),
        name: String::from("Rust"),
    };

    let mut course2 = Course {

        name:  String :: from("English"),
        level: String :: from("Intermidiate"),
        code:  240;
    };

    println!("Name: {}, Level: {}, Code: {}", course1.name, course1.level, course1.code);
    println!("Name: {}, Level: {}, Code: {}", course2.name, course2.level, course2.code);

    //update
    course1.name = String::from("Urdu");
    course1.level = "NoLevel".to_string();
    course1.level = 44;


    println!("Name: {}, Level: {}, Code: {}", course1.name, course1.level, course1.code);


}

// Output
// 1.55s

// Name:Rust, Level:beginner, code: 130
// Name:Javascript, Level:beginner, code: 122
// Name:Urdu, Level:NoLevel ,code: 44
