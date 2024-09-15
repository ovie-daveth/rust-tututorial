pub fn learn(){

    // Copy: both x and y have different memory location, this is different from borrowing
    // let mut x = 5;  // i32 implements Copy
    // let mut y = x;  // x is copied to y

    // println!("ref of x is: {:p}, and y is: {:p}", &x, &y);

    // //borrowing: both x and y have access to the same memory
    // let z = 54;
    // let d = &x;

    // println!("Testing if d still have access {}", d);  //will give error
     
}