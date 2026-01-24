use std::ops;

trait Summary{
    //? default implementation
    fn summarize(&self)-> String{
        return String::from("Hi there");
    }
}

struct User{
    name: String,
    age:u32,
}

struct Fix {}

impl Summary for Fix {}

impl Summary for User{
    //? if this implementation is not there then complier won't complain and use the default implementaion 
    fn summarize(&self)-> String {
        return format!("The name is {} and the age is {}", self.name , self.age);
    }
}
// ! New Learning
trait Shape {
    fn area(&self)-> u32;
}

struct Rectangle{
    width: u32,
    height: u32
}

struct Circle{
    radius : f64
}

impl Shape for Rectangle {
    fn area(&self)-> u32 {
        self.height * self.width
    }
}

impl Shape for Circle {
    fn area(&self)-> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

// ! Overload traits

// ? Add trait
struct A;
struct B;
#[derive(Debug)]
struct AB;
#[derive(Debug)]
struct BA;

impl ops::Add<B> for A {
    type Output = AB;

    fn add(self, rhs: B) -> AB {
        AB
    }
}

impl ops::Add <A> for B{
    type Output = BA;

    fn add(self, rhs: A) -> BA {
        BA
    }
}

// ? Drop Trait
struct C {
    c: String
}

impl Drop for C {
    fn drop(&mut self) {
        println!("dropped {}", self.c)
    }
}

fn main(){
    let user = User{
        name: String::from("Vedik"),
        age:21,
    };
    println!("{}", user.summarize());

    let f = Fix{};
    notify(f);

    // ! New Learning
    let r = Rectangle{
        width: 747,
        height: 987
    };
    let c = Circle {
        radius: 89.3
    };

    println!("Rect: {}, Circle: {}", r.area(), c.area());

    // ! Overload trait
    // ? Add trait
    println!("{:?}", A + B);
    println!("{:?}", B + A);

    // ? Drop trait
    let a =  C{c: String::from("a")};
    {
        let b =  C{c: String::from("b")};

        {
            let c =  C{c: String::from("c")};

            println!("leaving inner scope 2")
        }
            println!("leaving inner scope 1")
    }

    drop(a);
    println!("program ending")

} 

// # Traits as parameters
fn notify(u: impl Summary){
    println!("{}", u.summarize())
}