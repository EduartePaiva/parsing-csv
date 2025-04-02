use std::{
    fs::File,
    io::{Read, Write},
};

pub fn read_the_file(path: &str) -> String {
    let mut input: String = String::new();
    File::open(path)
        .expect("error opening the file")
        .read_to_string(&mut input)
        .expect("Some error converting to string");
    input
}

pub fn save_the_file(path: &str, text: String) {
    let mut file =
        File::create(path).expect(format!("error creating file for path: {}", path).as_str());
    write!(file, "{}", text).expect("error saving the file");
}
