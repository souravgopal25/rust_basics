/*
    Variables hold primitive data
    Immutable by default;
    Rust is block-scoped language
*/

pub fn run(){
    let name ="Sourav";
    let mut age=20;
    age =age+1;
    println!("My name is {} and I am {}",name,age);
    //Define Consatnt
    const ID:i32 =001;
    println!("ID :{}",ID);
    let( my_name ,my_age)=("Sourav",21);
    println!("My name is {} and I am {}",my_name,my_age);
}