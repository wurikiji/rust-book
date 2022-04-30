trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// fully qualified syntax is needed

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        "puppy".to_owned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn human_can_fly() {
        let person = Human {};
        person.fly();

        Pilot::fly(&person);
        Wizard::fly(&person);
    }

    #[test]
    fn baby_of_animals() {
        println!("A baby dog is called a {}", Dog::baby_name());
        // fully qualified syntax
        println!(
            "A baby dog is also called a {}",
            <Dog as Animal>::baby_name()
        );
    }
}
