use std::fs;

fn main() {
    let contents = fs::read_to_string("hello.txt");

    match contents {
        Ok(contents) => {
            println!("File read successfully: {:?}", contents);
        },
        Err(e) => {
            println!("Failed to read file: {:?}", e);
        }
    }
}