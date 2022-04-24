fn immutable_and_mutable() {
	let mut s = String::from("hello");

	let r1 = &s; // no problem
	let r2 = &s; // no problem

	let r3 = &mut s; // BIG PROBLEM

	println!("{}, {}, and {}", r1, r2, r3);
}

fn multiple_mutable_borrowing() {
	let r1 = &mut s2;
	let r2 = &mut s2;
	println!("{}, {}", r1, r2);
}
