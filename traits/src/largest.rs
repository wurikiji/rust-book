pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &value in list {
        if value > largest {
            largest = value;
        }
    }
    largest
}

pub fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for value in list {
        if *value > *largest {
            largest = value;
        }
    }
    largest
}
