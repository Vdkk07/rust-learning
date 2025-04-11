// # Sturcts with refrences
struct User<'a>{ //? lifetime of user is smallest among these two
    name: &'a str,
    password: &'a str,
}

fn main() {
    // # struct with lifetimes
    let u_name = String::from("vedik");
    let u_password = String::from("12345");

    let u = User{
        name: &u_name,
        password: &u_password
    };

    println!("{} {}", u.name, u.password);

    // # string with lifetimes
    let longest_str;
    let str1 = String::from("Hello");
        let str2 = String::from("Rust");

    {
        let str3 = String::from("");
        longest_str = longest_string( &str1,  &str2, &str3); // ? rust won't complie it "`str3` does not live long enough"
    }
    // ! longest_str will be a dangling pointer if str2 > str1
    println!("{}", longest_str);

}

fn longest_string<'a, 'b>(str1: &'a String, str2: &'a String, str3: &'b String) -> &'a String { 
    if str1.len() > str2.len() {
        return  &str1;
    } else {
         &str2
    }
}





