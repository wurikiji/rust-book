fn main() {
    let s = "hello";
    println!("str literal {}", s);
    // dynamic string is not known at compile-time
    let mut s = String::from("hello");
    println!("String from str literal {}", s);

    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("x {} y {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1;
    // impossible because s1 is moved
    // println!("s1 {}", s1);
    println!("s2 {}", s2);

    let s3 = s2.clone();
    println!("s2 after cloning {}, s3 {}", s2, s3);

    takes_ownership(s3);

    // s3 is now moved
    // println!("Can not use s3 anymore {}", s3);
    makes_copy(x);

    println!("Still can access x {}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{} and {}", s1, s3);

    let can_i_do_this = {
        let s1 = String::from("hello world!");
        s1
    };
    println!("You can do this {}", can_i_do_this);
    println!("And the length is {}", calculate_length(can_i_do_this).1);
}

// String takes `move` approach
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

// primitives take `copy` approach
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Now it's yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
