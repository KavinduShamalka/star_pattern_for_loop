pub fn box_pattern() {

  for _ in 1..5 {

    for _ in 1..5 {
      print!("*");
    }
    println!();
  }

}

/*
      *
     * *
    * * *
   * * * *
  * * * * *
*/
pub fn pyramid1() {

  let mut x = 1;

  let mut z = 5;

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
}

/*

*****
****
***
**
*
 
*/
pub fn pyramid2() {

  let mut x = 6;

  //print rows
  for _ in 1..6 {

    //print stars
    for _ in 1..x {
      print!("*");
    }
    println!();
    x = x - 1;

  }
}




/*

*
**
***
****
*****

*/
pub fn pyramid3() {
  let mut x = 6;
  let mut z: i32 = 1;

  for _ in 0..5 {
    for _ in 0..x {
      print!("");
    }

    for _ in 0..z{
      print!("*");
    }
    println!();
    x = x - 1;
    z = z + 1;
  }
}




/*

*****
****
***
**
*

*/
pub fn pyramid4() {
  let mut x = 6;
  let mut z: i32 = 1;

  for _ in 0..5 {

    for _ in 1..x{
      print!("*");
    }

    for _ in 0..z {
      print!("");
    }


    println!();
    x = x - 1;
    z = z + 1;
  }
}


/* 
 *****
  ****
   ***
    **
     *
*/
pub fn pyramid5() {

  let mut x = 5;
  let mut z: i32 = 1;

  for _ in 0..5 {

    for _ in 0..z {
      print!(" ");
    }

    for _ in 0..x {
      print!("*");
    }

    x -= 1;
    z += 1;

    println!();
  }

}

pub fn pyramid6() {

  let mut x = 5;

  let mut z = 1;

  for _ in 0..5 {

    for _ in 0..z {
      print!(" ");
    }

    for _ in 0..x {
      print!(" *");
    }

    x -= 1;
    z += 1;
    println!();
  }

  
}