fn main() {
    let v: Vec<i32> = Vec::new();
    // create vector using macro
    let v2 = vec![1, 2, 3];

    // should be mut
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    {
        let v = vec![1, 2, 3, 4];
        // dropped
    }

    // reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {third}");

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is not third element."),
    }
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);

    // reference error
    // println!("The first element is : {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
