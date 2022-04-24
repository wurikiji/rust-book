fn main() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else if number % 2 == 1 {
        println!("number is an odd");
    } else if number % 3 == 0 {
        println!("number is a multiple of three");
    } else {
        println!("number is an even");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is: {}", number);
    loop_loop();
    while_loop();
    for_loop();
}

fn loop_loop() {
    // inifinite loop
    loop {
        println!("again!");
        break;
    }

    let mut count = 0;
    // labeled loop
    'counting_up: loop {
        println!("hello {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // loop returning value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // error prone code,
    // index can reach outside of the range.
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("range value is: {}", number);
    }

    for number in (1..4).rev() {
        println!("count down: {}", number);
    }
}
