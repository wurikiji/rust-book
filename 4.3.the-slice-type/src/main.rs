fn main() {
    let s = String::from("Hello world!");
    let _word = first_word(&s);
    let _word = &s[0..2];
    let _word = &s[..2];
    let len = s.len();
    let _word = &s[2..len];
    let _word = &s[..];
    let word = first_word_v2(&s);
    // s.clear(); // error

    println!("Found {}", word);
    let s = "hello worlld!"; // string literal is a slice
    let word = first_word_v3(&s);
    println!("{}", word);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
