use std::{fs::File, io::ErrorKind};

pub fn unrecoverable_study() {
    let f = File::open("hello.txt");

    // let file1 = match f {
    //     Ok(ref file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    let file2 = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // clean up the match arms
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap();
    // customize panic statement
    // let f = File::open("Hello2.txt").expect("Failed to open hello2.txt");
}
