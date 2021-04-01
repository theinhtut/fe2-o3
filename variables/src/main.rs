fn main() {
    let mut m = 5;
    println!("The value of x is: {}", m);
    m = 6;
    println!("The value of x is: {}", m);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("You have {} space(s)", spaces);

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // // Char
    let c = 'z';
    let z = 'Ƶ';

    // // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}