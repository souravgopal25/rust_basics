//Functions

pub fn run(){
    greeting("Hello", "Sourav");
    //Bind functions value to variables
    let get_sum=add(5,4);
    println!("Sum : {}",get_sum);


    //closure
    let add_num=|n1:i32,n2:i32| n1+n2;
    println!("Sum of the closure {}",add_num(3,3));
}

fn greeting(greet:&str,name:&str){
    println!("{} {},nice to meet you! ",greet,name);
}

fn add(n1:i32,n2:i32)-> i32{
    n1+n2
}