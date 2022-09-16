Pass Structs to a Function

fn function_name(variable_name:variable_type){

    statement1;
    statement2;

    statementN;
}

so in this case 

fn function_name(struct_instance:struct_name){
    statement1;
    statement2;

    statementN;
}

//=======================Example======================


struct Course {

    code:i32,
    name:String,
    level:String,
}

fn display_mycourse_info(course:Course){
    println!(
        "course Id: {}, course Name: {}, course Level: {}.",
        course.code, course.name, course.level
    );
}

fn main (){
    
    let course1 = Course { 
        name:String::from("Rust"),
        level:"Expert".to_string(),
        code:32,
    };

    let  course2 = Course { 
        name:String::from("C++"),
        level:"littlebit high".to_string(),
        code:44,
    };

    display_mycourse_info(course1);
    display_mycourse_info(course2);

}

// Output
// 5.7s

// course Id: 32, course Name: Rust, course Level: Expert.
// course Id: 44, course Name: C++, course Level: littlebit high.


//=================Return Structs From a Function================

fn funnction_name (struct_instance:StructName) -> StructName{

    statement1;
    statement2;

    struct_instance //returns
}

//===================Example=========================

struct Course {

    code:i32,
    name:String,
    level:String,
}

fn display_mycourse_info(course:Course){
    println!(
        "course Id: {}, course Name: {}, course Level: {}.",
        course.code, course.name, course.level
    );
}

fn set_structure_values(course:Course) -> Course{

    course.code: 33;
    course.name: "Java".to_string();
    course.level: String::from("bigner"),

    course;
}

fn main (){
    
    let course1 = Course {};

    // display_mycourse_info(course1);
    // display_mycourse_info(course2);

}