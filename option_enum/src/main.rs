
fn main(){
   let ans = find_first_i(String::from("VedikSaini"));

   match ans {
       None => println!("Value not found"),
       Some(val) => println!("i found at index {}", val),
   }

   // ! Practicing
   let res = division(5.89, 3.98);

   match res {
       None => println!("Cannot divided by 0"),
       Some(x)=> println!("{}", x)
   }
}

fn division(x: f64, y: f64) -> Option<f64> {
   if y == 0.0{
      None
   }
   else {
       Some(x/y)
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