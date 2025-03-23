trait Summary{
    //? default implementation
    fn summarize(&self)-> String{
        return String::from("Hi there");
    }
}

struct User{
    name: String,
    age:u32,
}

struct Fix {}

impl Summary for Fix {}

impl Summary for User{
    //? if this implementation is not there then complier won't complain and use the default implementaion 
    fn summarize(&self)-> String {
        return format!("The name is {} and the age is {}", self.name , self.age);
    }
}

fn main(){
    let user = User{
        name: String::from("Vedik"),
        age:21,
    };
    println!("{}", user.summarize());

    let f = Fix{};
    notify(f);
} 

// # Traits as parameters
fn notify(u: impl Summary){
    println!("{}", u.summarize())
}