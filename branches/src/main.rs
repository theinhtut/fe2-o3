fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else  {
        println!("condition was false");
    }

    let number1 = 6;
    if number1 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number1 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number1 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }
}
