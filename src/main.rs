fn main() {
    //constants are always immutable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // the mut keyword before a variable allows it to be mutable, variables are immutable if declared without this keyword.
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //redcclaring a immutable variable is known as shadowing, this allows us to use the same variable name while
    //ensuring that the new value is also immutable going forward.
    let x = x + 1;

    {
        //Shadowing obeys scoping rules, so a variable shadowed in a function will have it's old value outside of the
        //function.
        let x = x * 2;
        println!("The value of the inner x is: {}", x);
    }

    println!("The value of the outer x is {}", x);

    //Shadowing allows you to change data types as well
    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces is: {}", spaces);
}
