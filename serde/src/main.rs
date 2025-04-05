use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
   username: String,
   password: String
}

fn main(){
    let u = User{
    username: String::from("vedik"),
    password: String::from("123456")
    };

    // Serialize
    let serialised_string = serde_json::to_string(&u);
    
    match serialised_string {
           Ok(json_str) => {
            println!("Serialized: {}", json_str);

            // deserialize
            let deserialized_result  = serde_json::from_str::<User>(&json_str);
            match deserialized_result {
                Ok(user) => println!("Deserialized: {:?}", user),
                Err(e) => println!("Error during deserialization: {}", e),
            }
        }
        Err(e) => println!("Error during serialization: {}", e),

    }
}