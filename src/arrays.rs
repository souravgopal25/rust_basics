//Arrays - Fixed list where elements are the same data types
pub fn run(){
    //syntax let arrayName:[dataType;length]=[elements];
    let mut numbers:[i32;5]=[1,2,3,4,5];
    //Reassigning Values
    numbers[2] = 20;
    //Debug print
    println!("{:?}",numbers);
    //get single value
    println!("{}",numbers[0]);
    //Memory allocated
    println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));
    //Get SLice
    let slice: &[i32]= &numbers[0..2];
    println!("{:?}",slice);
}