fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_largest(list: &[i32]) {
    let result = largest(list);
    println!("The largest number is {}", result);
}

pub fn find_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    print_largest(&number_list);
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    print_largest(&number_list);
}
