fn main() {
    let mut c = 0;

    loop {
        println!("finite");
        let _ = c += 1;

        if c >= 10 {
            break;
        }
    }

    // ! Label Loops
    'a: loop {
        println!("loop a");
        'b: loop {
            println!("loop b");
            'c: loop {
                println!("loop c");

                break 'b;
            }
        }
    }

    // ! Loop statement as binding
    let x = loop {
        break 10;
    };

    println!("x: {}", x);

    // ! while loop
    let mut n = 10;

    while n != 0 {
        println!("n: {}", n);

        n -= 1;
    }

    // ! for loop
    let v = vec![10, 20, 30, 40, 50];

    for i in v {
        println!("i: {}", i);
    }

    for i in 0..101 {
        println!("i: {}", i);
    }
}
