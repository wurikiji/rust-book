mod duplicated;
mod non_generic;
mod refactored;
mod with_generic;
fn main() {
    duplicated::find_largest();
    refactored::find_largest();
    let number_list = vec![34, 50, 25, 100, 65];
    let result = non_generic::largest_i32(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = non_generic::largest_char(&char_list);
    println!("The largest char is {}", result);
}
