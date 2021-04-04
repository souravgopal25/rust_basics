pub fn run(){
    println!("Hello");
    let mut count=0;
   /* loop{
        count+=1;
        println!("Numbers :{}",count);
        if count == 20{
            break;
        }
    }*/

    //while loop (FizzBuzz)
    while count<=100{
        if count % 15==0{
            println!("FizzBuzz");
        }else if count %3 == 0{
            println!("Fizz");
        }else if count %5 ==0{
            println!("Buzz");
        }else{
            println!("{}",count);
        }
        count+=1;
    }

}