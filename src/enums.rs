//Enums are type with defininte value

enum Movement{
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m:Movement){
    match m{
        Movement::Up =>   println!("Avtar Moving Up"),
        Movement::Down => println!("Avtar Moving Down"),
        Movement::Left => println!("Avtar Moving Left"),
        Movement::Right =>println!("Avtar Moving Right"),
    }
}

pub fn run(){
    let avtar1=Movement::Left;
    let avtar2=Movement::Up;
    let avtar3=Movement::Right;
    let avtar4=Movement::Down;
    move_avatar(avtar1);
    move_avatar(avtar2);
    move_avatar(avtar3);
    move_avatar(avtar4);


}