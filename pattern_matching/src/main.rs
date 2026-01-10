fn main() {
    // ! example-1
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    // ! example-2
    let n = 15;

    match n {
        1 => println!("one"),
        3 | 5 | 7 | 11 => println!("this is a prime"),
        13..=19 => println!("teens"),
        _ => println!("ain't special"),
    }

    // ! example-3 (pattern matching in tupple)
    let pair = (0, -2);

    match pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("no match"),
    }

    // ! example-4 (tupples and guards)
    let pair2 = (4, -4);

    match pair2 {
        (x, y) if x == y => println!("both are equal"),
        (x, y) if x + y == 0 => println!("equal zero"),
        (x, y) if x % 2 == 0 => println!("x is even"),
        _ => println!("no match"),
    }

    // ! example-5 (tupples and guards)
    let p = 5;

    let n = match p {
        n @ 1..=12 => n,
        n @ 13..=19 => n,
        _ => 0,
    };

    println!("n: {}", n)

    //? n @ sign allow us to bind the variable whatever the match is. Very useful when we doing match on a value we don not have an ownership of, so its essentially allow us to clone the matched value
}
