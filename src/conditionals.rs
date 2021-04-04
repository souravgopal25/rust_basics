pub fn run(){
    let age:u8=30;
    let check_id:bool = true;

    if age>=21 && check_id{
        println!("BarTender : What would you like to have?");
    }else if age<=21 && check_id{
        println!("BarTender: Sorry you have to leave!");
    }else{
        println!("BarTender:I'll need to see your ID");
    }
}