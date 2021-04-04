pub fn run(){
    let mut hello =String::from("Hello");
    //Get Length
    println!("Length {}",hello.len());
    //push char
    hello.push('W');

    //push String
  //  hello.push_str("")

    //capacity in bytes
    println!("CAPACITY :{}",hello.capacity());
    println!("{}",hello);

}