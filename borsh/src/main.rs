use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User{
    username: String,
    password: String
}

fn main(){
    let u = User{
        username: String::from("Vedik"),
        password: String::from("1234567")
    };

    let mut v: Vec<u8> = Vec::new();

    // serialize
    let serialize = u.serialize(&mut v);

    match serialize {
        Ok(_) => {
            println!("Serialize: {:?}", v);

            // deserialize
            let deserialize_user = User::try_from_slice(&v);
            
            match deserialize_user {
                Ok(dsz_u) => {
                    println!("Deserialize: {:?}", dsz_u);
                }

                Err(_) => println!("Error while deserializing")
            }
        
        }
        Err(_) => println!("Error while serializing")
    }
}