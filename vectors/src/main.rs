fn main (){
    let mut vec = vec![1,2,3,4,5,6,7];
    println!("Orginal vector: {:?}", vec);

    //# Approach 1
    let even_vec = even_integer(&vec);
    println!("create new vector for even integers: {:?}",even_vec ); // passing vec as an immutable refrence

    println!("Orginal vector: {:?}", vec); 

    //# Approach 2
    even_integer2(&mut vec);
    println!("mutate the original vector for even integers: {:?} ",vec);


}

// # Approach 1 (create a new vec for storing the even integer)
fn even_integer(v: &Vec<u32>) -> Vec<u32> {

    let mut new_vec = Vec::new();

    for val in v {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}

// # Approach 2 (mutate the original vector for storing the even integer)
fn even_integer2(v: &mut Vec<u32>) {

   let mut i = 0;

   while i < v.len(){
    if v[i] % 2 != 0 {
        v.remove(i);
    }else {
         i += 1;
     }
   }

} 