fn main() {
    let x = 1;
    let equal_to_x = move |z| z == x;
    println!("can't use x here: {:?}", x);
    let y = 1;
    assert!(equal_to_x(y));
}
