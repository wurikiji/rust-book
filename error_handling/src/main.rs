use std::error::Error;

mod unrecoverable;

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("Crash");
    let v = vec![1, 2, 3];
    // v[99];
    unrecoverable::unrecoverable_study();
    Ok(())
}
