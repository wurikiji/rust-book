use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly..");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    fn expensive_job(num: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(expensive_job);
    // let mut expensive_result = Cacher::new(|num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // });

    if intensity < 25 {
        println!("Today, do {} pushupds!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T, Q, R>
where
    T: Fn(Q) -> R,
{
    calculation: T,
    values: HashMap<Q, R>,
}

impl<T, Q, R> Cacher<T, Q, R>
where
    T: Fn(Q) -> R,
    Q: Eq + Hash + Copy,
    R: Copy,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: Q) -> R {
        match self.values.get(&arg) {
            Some(v) => *v,
            _ => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_diffrent_type() {
    let mut c = Cacher::new(|a| a);
    let s = c.value("jiji");
    assert_eq!(s, "jiji");
}

#[test]
fn call_with_diffrent_return_type() {
    let mut c = Cacher::new(|a: &str| a.chars().count());
    let s = c.value("jiji");
    assert_eq!(s, 4);
}
