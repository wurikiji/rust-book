use std::fmt::{Debug, Display};

use crate::aggregator::Summary;

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_long_form<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_when_generic_is_suitable<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

pub fn notify_with_display_trait(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
}

pub fn notify_with_display_trait_long_form<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item);
}

pub fn notify_with_where_clause<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    33
}
