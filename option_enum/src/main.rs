
fn main(){
   let ans = find_first_i(String::from("VedikSaini"));

   match ans {
       None => println!("Value not found"),
       Some(val) => println!("i found at index {}", val),
   }
}

fn find_first_i(str:String)-> Option<u32>{
   
   let mut index = 0;
   for c in str.chars(){
       if c == 'i' {
          return Some(index + 1);
        }
        index = index + 1; 
   };

   None

}