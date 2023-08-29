fn main() {
    
    let row: i32 = 5;
    
    //itarate by adding new row
    for i in 1..=row {
      
      //Adding spaces
      for _ in 0..=row - i {
        print!(" ");
      }

      //Adding stars
      for _ in 0..i {
        print!("* ");
      }

      println!(""); //print new line
    }

}
