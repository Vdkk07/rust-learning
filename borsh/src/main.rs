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

    // serialize
    let encode = borsh::to_vec(&u).unwrap();
    println!("{:?}", encode);
    // deserialize m-1
    let decode:Result<User, std::io::Error> = borsh::from_slice(&encode);
    println!("{:?}", decode);
    // deserialize m-2
    let decode_2 = User::try_from_slice(&encode).unwrap();
    println!("{:?}", decode_2);
    
}