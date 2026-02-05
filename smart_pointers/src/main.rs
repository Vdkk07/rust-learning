//! Smart pointers are structs that behave like references and clean up automatically.
use std::ops::Deref;

use crate::List::{Cons, Nil};

//?Enabling Recursive Types with Boxes
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

//? Using Box<T> Like a Reference
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//? Implementing the Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    //! Using Box<T> to Point to Data on the Heap
    //? Storing Data on the Heap
    let b = Box::new(6);
    println!("{}", b);

    //?Enabling Recursive Types with Boxes
    let list = List::Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);

    // ! Treating Smart Pointers Like Regular References
    //?  Following the Reference to the Value
    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // assert_eq!(5, y); //# give an error:

    //? Using Box<T> Like a Reference
    let x = 5;
    let y = Box::new(x);
    let z = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    //? Using Deref Coercion in Functions and Methods
    
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }
}
