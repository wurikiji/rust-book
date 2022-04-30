use advanced_traits::{
    newtype_pattern::Wrapper,
    supertraits::{OutlinePrint, Point},
};

fn main() {
    let point = &Point { x: 1, y: 2 };
    point.outline_print();

    let w = Wrapper(vec![String::from("hello"), "world".to_owned()]);
    println!("w = {}", w);
    // use of Deref
    println!("length of w is {}", w.len());
}
