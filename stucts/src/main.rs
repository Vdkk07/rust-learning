use std::fmt;
#[derive(Debug)]
struct Rect{
    width: u32,
    height:u32,
}

// ! Display triat implementation
impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.width, self.height)
    }
}

// ! Unit Tuple Structs
struct Color(i32, i32, i32);

// ! Unit-Like Structs
struct AlwaysEqual;

// ! Methods 
impl Rect {
    fn area (&self) -> u32{
        return self.width * self.height;
    }

    fn perimeter (&self) -> u32{
        return  2* (self.height + self.width);
    }

    fn show (&self){
         println!("the area of rectange is {} " , self.area());

         println!("the perimeter of rectangle is {} ", self.perimeter());
    }
}

// ! Related Function
impl Rect {
    fn new(width: u32, height: u32) -> Rect {
        Rect {
            width,
            height
        }
    }
}

fn main(){
    let black = Color(0, 0, 0); //Using Tuple Structs
    let subject = AlwaysEqual; // Using unit struct

    let rect = Rect{
        width:20,
        height:30,
    };

    let rect2 = Rect::new(44, 56);

    rect.show();
    rect2.show();

    // ! Debug trait
    println!("{:#?}", rect); //? Pretty debug
    println!("{:?}", rect2);

    // ! Display triat
    println!("{}", rect); 
    println!("{}", rect2);
}