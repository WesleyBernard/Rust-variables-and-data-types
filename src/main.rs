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

    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

    // Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.
    //Integer division rounds down to the nearest integer. The following code shows how you’d use each numeric operation in a let statement:

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("Here come all those basic math solutions:\nSum:{}\nDiff:{}\nProduct:{}\nQuotient:{}\nFloored:{}\nRemainder:{}",sum, difference, product, quotient, floored, remainder);

    //rust has two primitive compound types, the first of which is called a tuple

    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    //destructuring can be used to assign the values in a tuple to multiple variables at once.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //here's how to declare an array. Note that all items in an array must be of the same type.

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //You can also initialize the array, this will fill the array with an initial value.

    let a = [3; 5]; //Is the same as writing let a = [3, 3, 3, 3, 3];

    //Arrays can be accessed like arrays in other languages, using indexing.

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

}
