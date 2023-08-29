fn main() {
    
    let row: i32 = 5;
    
    //itarate by adding new row
    for i in 1..=row {

      //printing spaces
      for _ in 0..=row - i {
        print!(" ");
      }

      //printing stars
      for _ in 0..i {
        print!("* ");
      }

      println!(""); //print new line
    }

}
