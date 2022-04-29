fn main() {
    // this does not compile, because let patterns match with irrefutables.
    // let Some(x) = Some(10);

    // this works
    if let Some(x) = Some(10) {
        println!("{}", x);
    };

    // compiler will warn this,
    // because irrefutables does not neccessarily need error handling
    if let x = 5 {
        println!("{}", x);
    };
}
