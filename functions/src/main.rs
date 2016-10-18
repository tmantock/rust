fn main() {
    let x: i32 = 5;
    let y: i32 = 68;
    print_number(y);

    let sum_to_print = sum_number(x,y);
    println!("{:?}", sum_to_print);

    // without type inference
    let f: fn(i32) -> i32 = plus_one;

    // with type inference
    let f = plus_one;

    let six = f(5);
    //diverges();
}

fn print_number(x: i32) {
    println!("x is {}", x);
}

fn sum_number(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum
}

fn plus_one(i: i32) -> i32 {
    i + 1
}

fn look_at_tuples() {
    //let x = (1, "hello");
    let x: (i32, &str) = (1, "hello");

    let mut z = (1, 2); // x: (i32, i32)
    let y = (2, 3); // y: (i32, i32)

    z = y;

    let (a, b, c) = (1, 2, 3);

    println!("x is {}", a);
}

// fn diverges() -> ! {
//     panic!("This function never returns!");
// }
