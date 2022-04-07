// derive keyword
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // only associated functions not method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// multiple impls are possbile
impl Rectangle {
    // same name with a field
    fn width(&self) -> bool {
        self.width > 0
    }

    // more arguments
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(1 * 50),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // output to err with filename, line number, and Debug derive
    dbg!(&rect1);

    println!("rect1 is {:#?}", rect1);
    println!("Rect's width is valid {}", rect1.width());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    dbg!(Rectangle::square(33));
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// tuple version
// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// naiive version
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
