type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}
// fn return_long_type() -> Thunk {}

type Result<T> = std::result::Result<T, std::io::Error>;

// two are same
fn generic<T>(t: T) {}
fn generic2<T: Sized>(t: T) {}

// release the Sized restriction
// To handle unsized objects,
// we should depend on &T type.
fn generic3<T: ?Sized>(t: &T) {}
