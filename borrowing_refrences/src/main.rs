//! Immutable Refrences
// fn main() {
//     let my_string = String::from("Hello, Rust!");
//     takes_ownership(&my_string);  // Pass a reference to my_string
//     println!("{}", my_string);    // This is valid because ownership was not transferred
// }

// fn takes_ownership(some_string: &String) {
//     println!("{}", some_string);  // some_string is borrowed and not moved
// }


// ! Mutable Refrences

// fn main() {
//     let mut s1 = String::from("Vedik");
//     update_word(&mut s1);
//     println!("{}", s1);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" RustPaglu");
// }

//! Mutable Refrences with immutable
fn main(){
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;

    s2.push_str(" World"); //if you use s2 just after borrowing and before declaring s3 (with mutable or immutable reference), the code will compile and run 

    let s3 = &s1;

    println!("{}",s3); 
}