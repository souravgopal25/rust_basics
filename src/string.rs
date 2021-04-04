pub fn run(){
    let mut hello =String::from("Hello");
    //Get Length
    println!("Length {}",hello.len());
    //push char
    hello.push('W');
    hello.push_str("orld");

    //push String
  //  hello.push_str("")

    //len in bytes
    println!("Lenght {}",hello.len());
    //capacity
    println!("CAPACITY :{}",hello.capacity());
    //replace
    println!("Replace :{}",hello.replace("World"," Sourav"));

    //loop through String by whiteSpaces
    for word in hello.split_whitespace(){
      println!("{} !",word);
    }

    
    
    //println!("{}",hello);

}