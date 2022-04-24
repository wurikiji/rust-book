fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        // 그냥은 > 연산자를 사용할 수 없음
        if item > largest {
            largest = item;
        }
    }
    largest
}

// two fields should be the same type
struct Point<T> {
    x: T,
    y: T,
}

// methods with generic
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// without generic
impl Point<i32> {
    fn y(&self) -> &i32 {
        &self.y
    }
}

// two fields can be different types
struct MultiPoint<T, U> {
    x: T,
    y: U,
}

// generics on impl and method don't need to be the same
impl<X1, Y1> MultiPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MultiPoint<X2, Y2>) -> MultiPoint<X1, Y2> {
        MultiPoint {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}
