enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Nil));
}
