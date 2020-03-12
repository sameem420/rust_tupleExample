fn main() {
    // defining tuple, also statically giving type
    let student_info:(&str,i32,i32) = ("Sam", 21, 4);
    println!("My name is : {}", student_info.0);
    println!("My age is : {}", student_info.1);
    println!("I have {} GF. LoL", student_info.2);

    // printing complete tuple
    println!("{:?}", student_info);

}
