fn main() {
    const MAX_POINTS: u32 = 100_000; // _ is a visual separator
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
    println!("The value of max_points is: {}", MAX_POINTS);

    // let can reassign different types to same variable
    let spaces = "   ";
    println!("print spaces: {}", spaces);
    let spaces = spaces.len();
    println!("reprint spaces as number: {}", spaces);

    let x = 2.0;
    println!("The value of x as float is: {}", x);
    let y: f32 = 3.0; // f32
    println!("The value of y as float is: {}", y);

    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("print 2 bools: {} {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("print chars: {} {} {}", c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);


    // access tuple elements with numbers
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // an array
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // define an arrays type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // init an array
    let a = [3; 5];  // equals let a = [3, 3, 3, 3, 3];


    let a = [1, 2, 3, 4, 5];
    let index = 10;

    // let element = a[index];  // will through an error during compilation
    // println!("The value of element is: {}", element);

    another_function(10);
    second_function(10, 5);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("Tha value of x is: {}", x);
}

fn second_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1       // semicolon does not work here. it needs an expression not a statement

}