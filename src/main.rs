fn main() {
    variables_and_mutability();
    tuple_type();
    array_type();
    function_example_one(3,4);
}

fn function_example_one(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    /* 
        let y = 6; is an expression that evaluates to the value 6
        Calling a macro is an expression.

        Expressions can be part of statements
        The block that we use to create new scopes, {}, is an expression 
        This block evaluates to 4
        Expressions do not include ending semicolons.
        If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
    */
    let y = {     
        let x = 3;
        x + 1 // note the no semi colon
    };
    println!("Value of y: {}", y);

    /*
        F.1
    */
    let fv = five();
    println!("Value of fv: {}", fv);

    /*
        F.2
    */
    let ps = plus_one(5);
    println!("Value of ps: {}", ps);
}

// F.1
/*
    We don’t name return values, but we do declare their type after an arrow (->).
    The return value of the function is synonymous with the value of the final expression in the block of the body of a function
    Can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
*/
fn five() -> i32 {
    5
}

// F.2
/*

*/
fn plus_one(x: i32) -> i32 {
    x + 1 // expression
    // x + 1; with the semicolon will get error because it is now a statement 
}


fn variables_and_mutability() {
        // constant
        const MAX_POINTS: u32 = 100;
        println!("x value: {}", MAX_POINTS);
    
        // mutable, variables are immutable by default
        let mut x = 5;
        println!("x value: {}", x);
        x = 6;
        println!("x value: {}", x);
    
        /* shadowing
            Shadowing is different from marking a variable as mut, 
            because we’ll get a compile-time error if we accidentally 
            try to reassign to this variable without using the let keyword. 
            By using let, we can perform a few transformations on a value 
            but have the variable be immutable after those transformations 
            have been completed.
    
            The other difference between mut and shadowing is that 
            because we’re effectively creating a new variable when 
            we use the let keyword again, we can change the type of 
            the value but reuse the same name. 
        */
        let y = 5;
    
        let y = y + 1;
    
        let y = y * 2;
    
        println!("The value of y is: {}", y);
    
        let spaces = " asdf  "; //string
        println!("The value of space is: {}", spaces);
        let spaces = spaces.len(); //integer
        println!("The value of space is: {}", spaces);    
}

fn tuple_type() {
    /*
        A tuple is a general way of grouping together a number of values 
        with a variety of types into one compound type. 
        Tuples have a fixed length: once declared, they cannot grow or shrink in size
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("tuple x: {0}, {1}, {2}", five_hundred, six_point_four, one);
}

fn array_type() {
    let array_1 = [1, 2, 3, 4, 5];
    println!("array_1: {}", array_1[0]);

    let array_2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array_1: {}", array_2[3]);

    let array_3 = [3; 5];
    println!("array_1: {0}, {1}, {2}, {3}, {4}", array_3[0], array_3[1], array_3[2], array_3[3], array_3[4]);
}

// fn data_types(){
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

//     // addition
//     let sum = 5 + 10;
//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;

//     // remainder
//     let remainder = 43 % 5;


//     let t = true;
//     let f: bool = false; // with explicit type annotation
// }