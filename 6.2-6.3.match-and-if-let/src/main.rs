#[derive(Debug, Clone)]
enum UsState {
    Alabama,
    Alaska,
    California,
    // -- skip --
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    let mut count = 0;
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::California));
    value_in_cents(Coin::Penny);
    let five = Some(5);
    let _six = plus_one(five).unwrap();
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        8 => (), // nothing happend by unit value
        other => move_player(other),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
