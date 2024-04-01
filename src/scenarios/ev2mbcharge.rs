use std::mem;
fn main() {
    // Your code here
    
    // Measure the size of a variable
    let my_variable: i32 = 42;
    let size_of_variable = mem::size_of_val(&my_variable);
    println!("Size of my_variable: {} bytes", size_of_variable);
    
    // Measure the size of a data structure
    struct MyStruct {
        field1: i32,
        field2: f64,
    }
    let my_struct = MyStruct { field1: 10, field2: 3.14 };
    let size_of_struct = mem::size_of_val(&my_struct);
    println!("Size of my_struct: {} bytes", size_of_struct);
}
