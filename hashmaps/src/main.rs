use std::collections::HashMap;

fn main(){
    // # Learning syntax of hasmaps
    let mut users = HashMap::new();

    // insert something on hashmap
    users.insert(String::from("vedik"), 22);
    users.insert(String::from("vedansh"), 19);

    // get something from hashmap
    let first_users_age = users.get("vedik");

    match first_users_age {
        Some(age) => println!("first user age is: {}", age),
        None => println!("user not found")
    }

    // # Write a function that take a vector of tuples as an input and return a hashmap
    let input_vec = vec![
        (String::from("vedik"), 21),
        (String::from("vedansh"), 19)
    ];

   let hm = group_value_by_key(input_vec);
   println!("{:?}", hm);

   // ! New Learning

   let mut hm: HashMap<String, i32> = HashMap::new();

   hm.insert(String::from("random"), 19);
   hm.insert(String::from("string"), 14);

   for i in &hm{
    println!("{:?}", i);
   }

   match hm.get(&String::from("random")) {
        Some(&v) => println!("{}", &v),
        _ => println!("no match")
   }

   hm.remove(&String::from("string"));

   for i in &hm{
    println!("{:?}", i);
   }
   
}

fn group_value_by_key(vec:Vec<(String,i32)>) -> HashMap<String,i32> {
    let mut hm = HashMap::new();

    for (key, value) in vec {

        // ! The hashing algorithm may place elements in different locations depending on internal optimizations.
        hm.insert(key, value);  // output have two possibilities - {"vedansh": 19, "vedik": 21} or {"vedik": 21, "vedansh": 19}
    }

    return hm;
}