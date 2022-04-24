fn main() {
    println!("Hello, world!");
    another_function();
    parameter_function(5);
    print_labeled_measurement(5, 'h');
    expression();

    let x = return_value();
    println!("The value of x is: {}", x);
    let w = plus_one(5);
    println!("The w plus one is: {}", w);
}

fn another_function() {
    println!("Another function.");
}

fn parameter_function(x: i32) {
    println!("Teh value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn return_value() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
