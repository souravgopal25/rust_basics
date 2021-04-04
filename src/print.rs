

pub fn run(){
    println!("Hello, World! from print.rs");
    //Basic Formating
    println!("Number : {}",1);
    //Basic Formating
    println!("{} + {} = {}",1,1,2);

    //positional Argument
    println!("{0} is from {1} and {0} likes to {2}","Sourav","India","Code");
    //Named Argument
    println!("{name} likes to play {sports}",name="Sourav",sports="Cricket");

    //placeholder traits
    println!("Binary : {:b}",10);

    //placeholder for Debug traits
    println!("{:?}",(12,true,"hello"));

    //Basic Maths
    println!("{} + {} = {} ",10,10,10+10);
}