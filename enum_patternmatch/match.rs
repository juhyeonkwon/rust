#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter from {:?}", state);
            25
        },
        // _ => (), 로 아무것도 매치 안될때 디폴트 비슷하게 쓸 수 있을 듯 함
    }

}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {

    let coin = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_cents(&coin));

    println!("{:#?}", coin);

    let five = Some(5);
    let six = plus_one(five);

    if let six = Some(6) {
        println!("six");
    }

    
}