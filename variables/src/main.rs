fn main() {
    const MAX_POINTS: u32 = 100_000;
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
    // let spaces = "   ";
    // let spaces = spaces.len();
}