fn main() {
    let mut z: i32 = 5;
    println!("z is: {:?}", z);
    z = 10;
    println!("z is now: {:?}", z);
    let (x,y) = (1,4);
    println!("This is not a tuple but a pattern: {:?}", (x,y));

    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // println!("The value of x is {} and value of y is {}", x, y); // This won't work

    let x: i32 = 8;
    {
        println!("{}", x); // Prints "8"
        let x = 12;
        println!("{}", x); // Prints "12"
    }
    println!("{}", x); // Prints "8"
    let x =  42;
    println!("{}", x); // Prints "42"

    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    println!("{}", x);
    let x = x; // x is now immutable and is bound to 7
    println!("{}", x);

    let y = 4;
    println!("{}", y);
    let y = "I can also be bound to text!"; // y is now of a different type
    println!("{}", y);
}
