pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[test]
fn greeting_contains_name() {
    let name = "Carol";
    let result = greeting(name);
    assert!(
        result.contains(name),
        "Greeting did not contain name, value was '{}'",
        result,
    );
}
