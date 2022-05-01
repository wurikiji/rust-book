//! `Fn` is trait, `fn` is a type
//! `fn` implements `Fn`, `FnMut`, and `FnOnce`
//! thus `fn` can be used in anywhere `Fn` is used.
//! But with C interop, `fn` should be used
//! because C doesn't have closures

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    _Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    let fp = returns_function_pointer();
    println!("Result: {}", fp(11));
    let fp = returns_closure();
    println!("Result: {}", fp(11));
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn returns_function_pointer() -> fn(i32) -> i32 {
    // if the closures do not capture
    // then it can be coerced to `fn`
    |x| x + 1
}
