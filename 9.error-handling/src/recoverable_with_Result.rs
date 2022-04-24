use std::{
    fs::{self, File},
    io::{self, Read},
    result::Result,
};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut f = File::open("Hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
