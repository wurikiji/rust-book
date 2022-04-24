use traits::{
    aggregator::{Summary, Tweet},
    methods::Pair,
    notify::{notify, notify_long_form},
    returning_type::returns_summarizable,
};

fn main() {
    let tweet = Tweet {
        username: String::from("Horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize2());
    notify(&tweet);
    notify_long_form(&tweet);

    let tweet = returns_summarizable();
    println!("1 new tweet: {}", tweet.summarize());

    let pair = Pair::new(10, 20);
    pair.cmd_display();
}
