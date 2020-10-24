use std::collections::HashMap;

fn main() {
    //TODO: Exercises
    // let mut numbers = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    // println!("Mean: {}", find_mean(&numbers));
    // println!("Median: {}", find_median(&numbers));
    // println!("Mean: {}", find_mean(&numbers));
    vector();
    string_type();
    hash_map_ownership();
}

// fn find_mean(nums: &[i32]) -> i32 {
//     1
// }

// fn find_median(nums: &[i32]) -> i32 {
//     1
// }
// fn find_mode(nums: &[i32]) -> i32 {
//     1
// }

fn vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];

    println!("The third elemet is {}", third);

    match v1.get(2) {
        Some(third) => println!("The third elemet is {}", third),
        None => println!("There is no third element."),
    }
}

fn string_type () {
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // This is interesting
    println!("s3 is {}", s3);

    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");
    let a = format!("{}-{}-{}", a1, a2, a3);
    println!("a: {}", a);
}

fn hash_map_ownership() {
    let field_name = String::from("Fav color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("score: {:?}", score);

    // Iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}", count);    
    }
    println!("{:?}", map);
}
