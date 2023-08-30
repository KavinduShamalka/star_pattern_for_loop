pub fn star(){
   
   let mut i = 1;
   let mut n = 7;

   while i <= 7 {
    
      let mut j = 1;
      while j <= n {
        print!(" ");
        j += 1;
      }

      n -= 1;

      for _ in 0..i {
        print!("* ");
      }
    
    println!();
    i += 1;
   }
}