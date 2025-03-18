fn main() {
    let mut my_string = String::from("vedik");
    let mut my_string2 = String::from("vedik");

    takes_ownership(my_string);
    my_string2 = takes_ownership_and_give_back(my_string2);
    println!("HI {} from orginal owner", my_string2); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) {
    println!("HI {} from new owner", some_string); // `some_string` now owns the data.
}

fn takes_ownership_and_give_back(some_string: String)-> String{
        println!("HI {} from temporary owner", some_string); 
        return some_string

}