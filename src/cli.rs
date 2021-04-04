use std::env;
pub fn run(){
    let args:Vec<String>=env::args().collect();
    println!("Args :{:?}",args);

    let command =args[1].clone();
    let name ="Sourav";

    if command =="hello"{
        println!("Hi {}, how are you?",name);
    }
}