fn main() {
    variables_and_mutability();
    tuple_type();
    array_type();
    function_example_one(3,4);
    control_flow();
    ownership();
}

fn ownership(){
    let s = "hello";

    // String. This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    let t = String::from("hello");
    // The double colon (::) is an operator that allows us to namespace this particular from function under the String type rather than using some sort of name like string_from. 

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // the memory is automatically returned once the variable that owns it goes out of scope.

    // ERROR
    // let s1 = String::from("hello");
    // let s2 = s1;
    
    // println!("{}, world!", s1);


    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
    let s1 = String::from("hello");
    let s2 = s1.clone(); //deep copy of the heap data of the String, not just the stack data

    println!("s1 = {}, s2 = {}", s1, s2);

    let heapString = String::from("hello");  // s comes into scope

    takes_ownership(heapString);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let stackX = 5;                      // x comes into scope

    makes_copy(stackX);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn control_flow(){
    // 'if' Expressions
    let number: i32 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // comment out to move farther
    // let errorNum: i32 = 3;

    // if errorNum {
    //     println!("hello");
    // }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    
    println!("The value of number is: {}", number);

    // will error
    // let condition = true;

    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };

    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    // INFINITE LOOP
    // loop {
    //     println!("again!");
    // }
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;

    // compiler adds runtime code to perform the conditional 
    //  check on every element on every iteration through the loop.
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let mut number = 3;

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
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