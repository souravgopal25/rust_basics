//Structs - used to create custom data types
//Traditional way
struct Color{
    red:u8,
    green:u8,
    blue:u8
}

//tuple struct
struct Color1(u8,u8,u8);
pub fn run(){
    let mut c=Color{
        red:255,
        green:0,
        blue:0
    };
    println!("Color :{} {} {}",c.red,c.green,c.blue);
    let mut c2=Color1(255,0,0);
    println!("Color : {}, {} ,{}",c2.0,c2.1,c2.2);


}