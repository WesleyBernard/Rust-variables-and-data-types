fn main() {
    //constants are always immutable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // the mut keyword before a variable allows it to be mutable, variables are immutable if declared without this keyword.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
