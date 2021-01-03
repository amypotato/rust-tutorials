fn main() {
    //mut keyword lets us change a variables value directly
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //example constant
    const MAX_POINTS: u32 = 100_000;

    let y = 10;
    println!("The value of y is: {}", y);

    // using let again creates an entirely new variable that "shadows y"
    let y = 6;
    println!("The value of y is: {}", x);

    // y = 12, uses previous value & creates shadow
    let y = y * 2;
    println!("The value of y is: {}", x);

    // assign a variable & shadow it with a new type
    let spaces = "   "; //string
    let spaces = spaces.len(); //number
    println!("The value of spaces is: {}", spaces);

    //the following will not compile
    //cannot change the type of a mutable variable
    let mut spaces = "   ";
    spaces = spaces.len();
}
