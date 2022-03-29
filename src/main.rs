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


    // Below are the different integer types in rust

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
    // So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127.
    // Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    //These are a few methods that can be used to handle integer overflow

    // Wrap in all modes with the wrapping_* methods, such as wrapping_add
    // Return the None value if there is overflow with the checked_* methods
    // Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    // Saturate at the value’s minimum or maximum values with saturating_* methods

}
