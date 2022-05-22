enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska
}

fn value_in_cents(coin:Coin) ->u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // 下面这使用match是繁琐，再下一段是使用 "if let"
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    }

    let some_u8_value1 = Some(0u8);
    if let Some(1) = some_u8_value1 {
        println!("one");
    }
}