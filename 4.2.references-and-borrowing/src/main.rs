fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    change(&mut s2);
    println!("Changed s2: {}", s2);

    // can not borrow mutable reference more than once
    // in a same scope
    /*
    let r1 = &mut s2;
    let r2 = &mut s2;
    println!("{}, {}", r1, r2);
    */

    {
        let r1 = &mut s2;
        println!("Inside {}", r1);
    }
    let r2 = &mut s2;
    println!(
        "This is possible because the precedent scope has been out {}",
        r2,
    );

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

// this can not be compiled
/*
fn wrong_change(some_string: &String) {
    some_string.push('a');
}
*/

// mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Dangling reference problem
/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
  */
