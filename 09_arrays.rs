//============= Arrays in Rust ==============


Define an Array 

let name: [type; size] = [element1, element2, element3];

fn main() {
   //define an array of size 4
   let arr:[i32;4] = [1, 2, 3, 4]; 
   //print the first element of array
   println!("The first value of array is {}", arr[0]);
   // initialize an array of size 4 with 0
   let arr1 = [0; 4]; 
   //print the first element of array
   println!("The first value of array is {}", arr1[0]);
}

Output
0.7s

The first value of array is 1
The first value of array is 0


Print the Array

fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //Using debug trait
    println!("\nPrint using a debug trait");
    println!("Array: {:?}", arr);
}

Output
0.69s


Print using a debug trait
Array: [1, 2, 3, 4]


Get the Length of the Array

fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    // print the length of array
    println!("Length of array: {}", arr.len());
}

Get Slice

fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //define the slice
    let slice_array1:&[i32] = &arr;
    let slice_array2:&[i32] = &arr[0..2];
    // print the slice of an array
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);
}


Output
0.7s

Slice of an array: [1, 2, 3, 4]
Slice of an array: [1, 2]
