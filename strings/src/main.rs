fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("Initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    let s2 = "bar";
    s.push_str(s2);
    s.push('j');

    // see the api doc for add() operation
    let sss = String::from("baz");
    let s4 = format!("{}-{}-{}", s, s2, sss);
    let s3 = s + s2 + &sss;
    // println!("s is not valid anymore {}", s);
    println!("Hello, world! {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    // indexing 을 지원하지 않음.
    // utf-8 을 dynamic 하게 저장하기 위함.
    // let f = s3[0];
    let f = &s3[0..1]; // 올바른 방식

    // 아래는 패닉을 발생시킴.
    // '이' 는 3바이트 문자라서, 0..3 으로 접근해야함.
    // let panic_string = "이것은패닉";
    // let s = &panic_string[0..1];
    for c in "이것은패닉이아님".chars() {
        print!("{}", c);
    }
    println!();
    for c in "이것도패닉이아님".bytes() {
        // 하지만 bytes 단위로 끊어서 u8 숫자로 출력됨.
        print!("{}", c);
    }
    println!();
}
