pub trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

// There will be only one implmentations
impl MyIterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(32)
    }
}

pub trait MyGenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

// There will be multiple MyGenericIterator implementations
impl MyGenericIterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(11)
    }
}
impl MyGenericIterator<String> for Counter {
    fn next(&mut self) -> Option<String> {
        Some("asdfasdf".to_owned())
    }
}
