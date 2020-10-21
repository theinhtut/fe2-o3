enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Coin value: {:?}", value_in_cents(Coin::Dime));
    println!("Coin value: {:?}", value_in_cents(Coin::Penny));
    println!("Coin value: {:?}", value_in_cents(Coin::Nickel));
    println!("Coin value: {:?}", value_in_cents(Coin::Quarter));

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
