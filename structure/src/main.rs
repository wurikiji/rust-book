struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit struct
struct AlwaysEqual;

fn main() {
    let user1 = User {
        email: String::from("wurikiji@gmail.com"),
        username: String::from("wurikiji"),
        active: true,
        sign_in_count: 1,
    };
    let mut user2 = User {
        email: String::from("wurikiji@gmail.com"),
        username: String::from("wurikiji"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("wurikiji@gmail.com");
    println!("{}", user2.email);
    println!(
        "{}",
        build_user(String::from("hi@gmail.com"), String::from("hiho")).email,
    );
    println!(
        "{}",
        build_user2(String::from("hi@gmail.com"), String::from("hiho")).email,
    );

    let user3 = User {
        email: String::from("User3@gmail.com"),
        ..user2
    };
    println!("{}", user3.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
