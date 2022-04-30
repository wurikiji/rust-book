use core::slice;

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is : {}", *r2);

        dangerous();
    }
    // dangerous(); // unsafe

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    let (a, b) = my_split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // unsafe logic
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r2, 1000) };
    // println!("{:?}", slice);
    // end of unsafe logic

    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to c: {}", abs(-3));
    }

    // global static constants
    println!("name is: {}", HELLO_WORLD);

    // accessing and modifying static mutable is unsafe
    unsafe {
        COUNTER += 3;
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..]) // borrow checker failed
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// unsafe trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}
