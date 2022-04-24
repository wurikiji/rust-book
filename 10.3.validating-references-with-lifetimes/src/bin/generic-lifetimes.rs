fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("Long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest(&string3, &string4);
        println!("The longest string is {}", result);
    }

    /*
    /// cant run this because string2 has shorter lifetime
    let string1 = String::from("short");
    let result;
    {
        let string2 = String::from("xx");
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);
    */
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn _always_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

/*
/// This function can't be compiled.
/// Because returned reference is owned by this fucntion
fn cant_return<'a>(x: &str, y: &str) -> &'a str {
    &String::from("really long string!")
}
*/
