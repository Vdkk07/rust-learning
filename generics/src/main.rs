// ! Generics over structs
struct Rect <T> {
    width: T,
    height: T
}

impl <T : std::ops::Mul<Output = T> + Copy> Rect<T> {
    fn area(&self) -> T {
        return self.width * self.height;
    }
}

fn main(){
    //? Generics and trait bounds
    println!("{}", sum(2,4));

    // ? Generics over structs
    let r = Rect{
        width:10,
        height: 12
    };
    println!("{:?}", r.area());
    println!("{}", r.width);

    let r1 = Rect{
        width: 10.5,
        height: 34.42,
    };
    println!("{:?}", r1.area());


}

// ! Generics and trait bounds
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) ->  T{
    return a + b;
}   