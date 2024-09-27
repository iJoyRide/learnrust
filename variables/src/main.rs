fn main() {
    let mut x = 5;
    x = x + 1;
    println!("The value of x is: {x}");
    
    const Y: u32 = 5;

    const HOUR_IN_SECONDS: u32 = 60 * 60 * Y;

    println!("{Y} hours in seconds is: {HOUR_IN_SECONDS}");
}