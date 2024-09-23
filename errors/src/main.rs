use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    read_file();
}

fn read_file() -> Result<String, io::Error> {
    let mut data_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match data_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
