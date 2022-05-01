use hello_macro::HelloMacro;
use hello_macro_derive::{sql, HelloMacro};

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    let sql = sql!(SELECT * FROM posts WHERE id = 1);
    println!("sql is {}", sql);
}
