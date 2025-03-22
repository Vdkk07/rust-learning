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

}