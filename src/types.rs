/*
Primitive Types ------------------------------------------------
Integers u8,i8,u16,i16,u32,i32,u64,i64,u128,i128 (number of bits they take in memory)
floats  : f32.f64
boolean (bool)
characters (char)
Tuples
Arrays
*/

pub fn run(){
    println!("Hello");
    //f32 by default
    let y=2.4;
    //i32 by default
    let x=5;

    //add Explicit types
    let z: i64 =454324235423;
    

    //Find Max Sizes
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);
}