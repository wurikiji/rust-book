pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String {
        format!("(Read more...) {}", self.summarize())
    }
}
