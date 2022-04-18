use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name} and {field_value} are moved");

    let score = scores.get("Blue").unwrap();
    println!("The blue team's score is {score}");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite an old value with a new vlaue
    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // insert if not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(33);
    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
