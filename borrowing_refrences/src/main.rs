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
// fn main(){
//     let mut s1 = String::from("Hello");
//     let s2 = &mut s1;

//     s2.push_str(" World"); //if you use s2 just after borrowing and before declaring s3 (with mutable or immutable reference), the code will compile and run

//     let s3 = &s1;

//     println!("{}",s3);
// }

//! borrowing, iteration, references, destructuring and iterators
fn main() {
    let v = vec![
        2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 9,
    ];

    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }

    println!("{}", v[1]);
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}
