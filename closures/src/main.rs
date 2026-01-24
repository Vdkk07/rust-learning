//? Closure traits -> Fn()
fn run<F>(f: F)
//# F is a generic type parameter
where
    F: Fn(), //# F must be something that can be called like a function with no arguments
{
    f();
}

fn add3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

//? Struct with closures
struct A<F: Fn(i32) -> i32> {
    f: F,
}
fn main() {
    let add = |a, b| a + b;
    println!("{}", add(4, 5));

    //? Closures can capture variables from outside.
    let x = 10;
    let add_x = |n| n + x;
    println!("{}", add_x(8));

    //? Closure that take no values
    let p = || println!("Closure that take no values");
    p();

    // ! closures capture variables Rust chooses **automatically** -> Borrow, Mutable Borrow or Move (take ownership)
    //? Mutable borrow
    let mut c = 0;

    let mut inc = || {
        c += 1;
        println!("increment by 1: {}", c);
    };

    inc();
    inc();
    inc();

    //? Closure traits -> Fn()
    let p = || println!("hello from run function!");
    run(p);

    let x = |i| i * 10;
    println!("3 * 10 = {}", add3(x));

    //? Struct with closures
    let a = A { f: x };

    //? Closure with iterators
    let v = vec![1, 2, 3, 4];

    println!("{}", v.iter().any(|&x| x != 2));
}
