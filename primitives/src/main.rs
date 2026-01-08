//! Tuples
// fn main() {
//     let t = (7, 'v', true);
//     let f = (4, t);  //? Tuple inside a tuple

//     println!("{} {} {}" ,t.0 ,t.1 , t.2 );
//     println!("{:?}" ,t );

// }

// fn main() {
//     let t = (); //? Any function that doesn’t return anything actually returns an empty tuples
// }

//! Array
// use std::mem;
// fn main(){
//     let xs  = [1,2,3,4,5];
//     println!("{} {} {}" ,xs[0], xs.len(), mem::size_of_val(&xs) );

//     let ys = &xs[2..4]; //? Slices are ALWAYS references
//     println!("{:?}", ys);
// }

//! String
//? String are not techincal litreal type in rust instead strings are more like tuples and arrays in the sense that they are compound types
fn main() {
    let s = "String";

    let ss = String::from("String");
    let slice = &ss[0..3];
    println!("{}", slice);

    let sss = "String".to_string(); //? We can use to_string() method for converting slice into String
}
