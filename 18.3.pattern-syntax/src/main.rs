fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    };

    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // y is shadowed
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;
    match x {
        // multiple patterns
        1 | 2 => println!("one or two"),
        // matching ranges
        1..=5 => println!("one through five"),
        6 => println!("six"),
        _ => println!("something else"),
    }

    let x = 'c';
    // range matching also works for `char` values
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    // destructuring
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // shorthand
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // struct pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    };

    // destructuring enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x dirction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        _ => (),
    }

    // Destructuring Nested Structs and Enums

    let msg = Message::ShowColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ShowColor(Color::Rgb(r, g, b)) => println!(
            "Show the color as rgb: red {}, green {}, and blue {}",
            r, g, b,
        ),
        Message::ShowColor(Color::Hsv(h, s, v)) => println!(
            "Show the color as hsv: hue {}, saturation {}, and value {}",
            h, s, v,
        ),
        _ => (),
    };

    // mix and match
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -1 });

    foo(feet, inches);
    foo(x, y);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    // ignoring
    let _x = 5;
    let y = 10; // warning

    /* error */
    /*
    let s = Some(String::from("Hello!"));
    // ignored in compile time but bind into _s (moved)
    if let Some(_s) = s {
        println!("found a string");
    }
    // this refers to moved data
    println!("{:?}", s);
    */

    let s = Some(String::from("Hello!"));

    // ignored by both compiler and runtime
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        // ignoring remaining parts
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // .. should be unambiguous
        (first, .., last) => {
            println!("Some nubmers: {}, {}", first, last);
        }
    }

    // match gurads

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = HelloMessage::Hello { id: 5 };

    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
    let x = 3;
    // range matching in if statement
    if (1..=3).contains(&x) {
        println!("if matches");
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ShowColor(Color),
}

// ignoring
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum HelloMessage {
    Hello { id: i32 },
}
