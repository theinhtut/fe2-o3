fn main() {
    println!("Hello, world!");
    another_function(5, 6);


    // Statements and Expressions in Function Bodies
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    let x1 = plus_one(5);
    println!("The value of x1 is: {}", x1);

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


// Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> {
    x + 1
}