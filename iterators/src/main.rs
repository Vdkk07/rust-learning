fn main (){
    let v1 = vec![1,2,3,4,5];

    // # iter()
    let v1_iter= v1.iter(); // iter() method borrowed item (creates an immutable iterator over a collection)
    
     for val in v1_iter {
        println!("{}", val);
    }
    
    println!("v1: {:?}", v1); 
        
    // # iter_mut
    let mut v2 = vec![1,2,3,4,5];

    let v2_iter = v2.iter_mut(); // iter_mut() method mutably borrowed items (creates a mutable iterator over a collection)

    for val in v2_iter{
       *val *= 2;
        println!("{}", val);
    }

    println!("v2: {:?}", v2);

    // # next()
    let v3 = vec![1,2,3,4,5];
    let mut v3_iter = v3.iter();

    while let Some(val) = v3_iter.next(){  //next() return you an Option
        println!("{}", val);
    }

    println!("v3: {:?}", v3); 

    // # into_iter()
    let v4 = vec![1,2,3,4,5];
    println!("v4: {:?}", v4); 

    let v4_iter = v4.into_iter(); // take the ownership of the collections

    for val in v4_iter{
        let  mul_val = val * 3;
        println!("{}", mul_val);
    }

    // ! v4 cannot be used because it is no longer the owner
    // println!("v4: {:?}", v4); 

    // # Consuming adapters -> method define on the iterator trait that consume the iterator
    let v5 = vec![1,2,3,4,5];
    let v5_iter = v5.iter();

    let sum:i32 = v5_iter.sum();

    println!("sum is: {}", sum);

    // ! We cannot use the iterator here because its value get consumed 
    // println!("v5_iter: {:?}", v5_iter);

    // # Iterator adapters -> produce different iterator by changing some aspect of the original iterator
    // ? Map()
    let v6 = vec![1,2,3,4,5];
    let v6_iter = v6.iter();
    let v6_iter_2 = v6_iter.map(|x| x + 1);

    for val in v6_iter_2 {
        println!("v6_iter_2 is {}", val);
    };

    // ? Filter()
    let v7 = vec![1,2,3,4,5];
    let v7_iter = v7.iter();
    let v7_iter_3 = v7_iter.filter(|x| *x % 2 == 0);

    for val in v7_iter_3 {
        println!("even values in v6_iter_3 is {}", val);
    }

    // # using collect() method on iterator for converting back to vector
    let v8 = vec![1,2,3,4,5];
    let v8_iter = v8.iter().filter(|x| *x % 2 == 1 ).map(|x| x * 2);

    let v8_vec: Vec<i32 >= v8_iter.collect();

    println!("new v8 vector: {:?}", v8_vec);

}