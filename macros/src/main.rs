use std::fmt::Display;

// ! Declerative Macros
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

// ! Procedural Macros
#[derive(Debug)] 
struct User {
    username: String,
	password: String,
	age: u32
}

impl Display for User{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "user {} is {} years old", self.username, self.age)
    }
}

// ! Copy, Clone macros
#[derive(Debug, Clone, Copy)]
struct User2{
    is_male: bool,
    age:u32,
}
fn main() {
    // ! Declerative Macros
    say_hello!();  // ? Expands to: println!("Hello, world!");

    // ! Procedural Macros
    let u = User{
        username: String::from("Vedik"),
        password: String::from("1234567890"),
        age: 21,
    };
    println!("{} {} {}", u.username, u.password, u.age);
    println!("{:?}", u); // # debug macro
    println!("{}", u); // # display macro

    // ! Copy, Clone macros
    // * (Copy trait implement by number but string don't)

    // # For numbers
    let u1 = 3;
    let u2 = u1; // ? copy the original variable 
    println!("u1:{} u2:{}", u1, u2);

    // # For String
    let u3 = String::from("vedik");
    let u4 = u3.clone(); // ? moved owenership of the variable, for using u3 after this line make a clone of it 
    println!("u3:{} u4:{}",u3, u4); 

    // # For structs 
    // * Rust don't know which varibale are you used inside the struct are they stored on stack or heap, if the variables which stored on stack like (numbers or bool) so we need to derive the Copy macro 

    // * if I added a string in struct then there is a problem becasue string stored on heap so you can't implement copy over here
    let u5 = User2{
        is_male: true,
        age:23,
    };  

    let u6 = u5;
    
    println!("{:?} {:?}", u5, u6 );
}




