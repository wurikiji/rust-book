use std::error::Error;

mod recoverable_with_Result;
mod unrecoverable;
mod validation;

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("Crash");
    let v = vec![1, 2, 3];
    // v[99];
    unrecoverable::unrecoverable_study();
    let username = recoverable_with_Result::read_username_from_file()?;
    let username = recoverable_with_Result::read_username_from_file_with_question()?;
    let username = recoverable_with_Result::read_username_from_file_shorter()?;
    let username = recoverable_with_Result::read_username_from_file_shortest()?;
    Ok(())
}
