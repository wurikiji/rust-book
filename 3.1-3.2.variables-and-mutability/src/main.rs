const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = x + 1;
    {
        let x = x * 2;
        // array type
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let arr = [arr[3]; 5];
        // tuple type
        let tup: (i32, i32, i32) = (x * 2, arr[0] * 3, x + 1);
        // tuple destruction
        let (x, ..) = tup;

        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value fo x is: {}", x);
    println!("Constant is {}", THREE_HOURS_IN_SECONDS);
}
