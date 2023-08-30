// mod test;
mod star_patterns;

fn main() {

    let mut x = 1; //stars
    let mut z = 5; //spaces
  
    for _  in 0..5{
  
      for _ in 0..z {
        print!(" ");
      }
  
      for _ in 0..x {
        print!(" *");
      }
  
      println!();
      z -= 1;
      x += 1;

    }
    // test::star();

    // star_patterns::box_pattern();
    // star_patterns::pyramid2();
    // star_patterns::pyramid3();
    // star_patterns::pyramid4();
    // star_patterns::pyramid5();
    // star_patterns::pyramid6();
    // star_patterns::pyramid1();
}
