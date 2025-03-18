struct Rect{
    width: u32,
    height:u32,
}

// ! Unit Tuple Structs
struct Color(i32, i32, i32);

// ! Unit-Like Structs
struct AlwaysEqual;

impl Rect {
    fn area (&self) -> u32{
        return self.width * self.height;
    }

    fn perimeter (&self) -> u32{
        return  2* (self.height + self.width);
    }
}

fn main(){
    let black = Color(0, 0, 0); //Using Tuple Structs
    let subject = AlwaysEqual; // Using unit struct

    let rect = Rect{
        width:20,
        height:30,
    };

    println!("the area of rectange is {} " , rect.area());

    println!("the perimeter of rectangle is {} ", rect.perimeter());
}