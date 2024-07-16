// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {

    let a:[i32;100] = [0;100];
    // numbers-数组的名称
    // [i32; 100]-表示数组的数据类型i32和大小100
    // [0; 100]-是一个重复表达式，这里的值0将填充数组100次

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
