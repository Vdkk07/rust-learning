// # Sturcts with refrences
struct User<'a> {
    //? lifetime of user is smallest among these two
    name: &'a str,
    password: &'a str,
}

fn main() {
    // # struct with lifetimes
    let u_name = String::from("vedik");
    let u_password = String::from("12345");

    let u = User {
        name: &u_name,
        password: &u_password,
    };

    println!("{} {}", u.name, u.password);

    // # string with lifetimes
    let longest_str;
    let longest_str_2;
    let longest_str_3;

    let str1 = String::from("Hello");
    let str2 = String::from("Rust");

    {
        let str3 = String::from("");
        longest_str = longest_string(&str1, &str2, &str3);
        longest_str_2 = longest_string_2(&str1, &str3); // ? rust won't complie it "`str3` does not live long enough"
        longest_str_3 = longest_string_3(&str1, &str2, &str3); // ? rust won't complie it "`str3` does not live long enough"
    }
    // ! longest_str will be a dangling pointer if str2 > str1
    println!("{}", longest_str);
    println!("{}", longest_str_2); // getting error
    println!("{}", longest_str_3); // getting error
}

// !  even though i have three argumemt but never returns the third one, here I if specific a single lifetime then rust will assume str3 may return in few cases and hence i will not let return anything
fn longest_string_3<'a, 'b>(str1: &'a String, str2: &'a String, str3: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        return &str1;
    } else {
        &str2
    }
}

// ! so because of the above conditon we have to specific two lifetimes
fn longest_string<'a, 'b>(str1: &'a String, str2: &'a String, str3: &'b String) -> &'a String {
    if str1.len() > str2.len() {
        return &str1;
    } else {
        &str2
    }
}

// ! if i've multiple variable with the  same lifetime then rust assume and considered the smallest lifetime among all the variables
// ? 'a is assumed the smallest lifetime among all of them
fn longest_string_2<'a, 'b>(str1: &'a String, str3: &'a String) -> &'a String {
    if str1.len() > str3.len() {
        return &str1;
    } else {
        &str3
    }
}
