fn main(){
    let  word = String::from("vedik saini");
    println!("original string is: {}", word);

    let word2 =  first_word(&word);
    println!("mutable string is: {}", word2);
}

fn first_word(str: &String) -> &str {

    let mut index = 0;
    for i in str.chars() {
        if i == ' '{
            break;
        }
        index = index + 1;
    }

    return &str[0..index];
}