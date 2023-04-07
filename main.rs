mod math;
//1.2 hello world
fn main(){
    // println!("Hello, world!")
    // var()
    // basic_op()
    // conditionals()
    // loop_fn()
    // for_fn()

    // let x = 5;
    // let y = 2;

    // let sum = math::adding(x,y);
    // let difference = math::subtracting(x,y);
    // println!("The sum of x and y is: {}", sum);
    // println!("The difference of x and y is: {}", difference);

    // struct_fn()

//     let p = Point {x: 3, y: 4};
//     println!("The distance from the origin is {}", p.distance_from_origin());
// enum_fn() 

// let a = 6;
// let b = 0;

// match divide(a, b) {
//    Ok(result) => println!("The result is: {}", result),
//    Err(e) => println!("Error: {}", e),
// }

// let number = 5;

// match find_even(number) {
//     Some(even) => println!("The even number is: {}", even),
//     None => println!("The number is not even"),
// }

// vec_fn()
// string_fn()
// move_fn()
borrow_fn()
}

    //Lesson 2: Variables, Data Types, and Basic Operations


//2.1 Variables
/**
In rust variables are immutable by default, 
use the mute keyword to declare mutable variables
*/

fn var() {
    let x = 5; //immutable variable
    println!("The value of x is: {}", x);

    let mut y = 7; //mutable variable
    println!("The value of y is: {}", y);

    y = 10;
    println!("The value of y is now: {}", y);
}

//2.2 Data types
/**
Rust has two categories of data types:
1. scalar
2. compound

Scalar- (integer, floating-point, boolean, character)
Compound- (Tuple-"collection of values with different type", Array-"collection of values with same type")

*/

//2.3 Basic Operations

fn basic_op(){
    let a = 5;
    let b = 2;

    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);
}


//Lesson 3: Control Flow

//3.1 Conditionals
/**
Rust uses if statements for conditional execution, as 
well as else if and else
*/

fn conditionals(){
    let x = 5;

    if x < 5 {
        println!("X is less than 5");
    } else if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is equal to 5");

    }
}

//3.2. Loops
//Rust has 3 types of loops: 'loop, while, for'

//3.2.1: loop
//The loop keyword creates an infinite loop, you can use
//the break keyword to exit the loop

fn loop_fn(){
    let mut counter = 0;

    loop {
        counter += 1;

        if counter > 5 {
            break;
        }
        println!("Counter: {}", counter);
    }
}

//3.2.2. while
//exeutes as long as the condition is true

fn while_fn() {
    let mut counter = 0;

    while counter < 5 {
        counter += 1;
        println!("Counter: {}", counter);
    }
}

//3.2.3. for
//used to loop through a range, array or other 
//iterable items

fn for_fn() {
    for i in 1..6 {
        println!("Counter: {}", i);
    }
}



//Lesson 4: Functions and Modules

//4.1. Functions
//Functions can have a return type, which is specified
//after the '->' symbol

fn add(a: i32, b: i32) -> i32 {
    a + b
}

//Lesson 5: Structs, Methods, and Enums

//5.1. Structs
// a custom data type that lets you group 
//related values together

struct Point {
    x: i32,
    y: i32,
}

fn struct_fn (){
    let p = Point { x: 5, y: 10};
    println!("The point is at ({}, {})", p.x, p.y)
}

//5.2. Methods
//Are functions that are associated with a struct.
//use the impl keyword folllowed by struct name and a block of code

impl Point {
    fn distance_from_origin(&self) -> f64 {
        let x_squared = (self.x * self.x) as f64;
        let y_squared = (self.y * self.y) as f64;

        (x_squared + y_squared).sqrt()
    }
}



//5.3. Enums
//A custom data type that rep a value that can be one of several variants

enum Color {
    Red,
    Green,
    Blue,
}

fn enum_fn() {
    let color = Color::Red;

    match color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}

//Lesson 6: Error Handling

//Rust handles errors using the 'Result' and 'Option' enums

//6.1 Result
/*
A result type is used to rep the outcome of a function that can return
an error. It has two variants: 'Ok(T)' for successfull results and
'Err(E)' for errors.
*/

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}


//6.2 Option
/*
Reps a value that can be either 'Some(T)' or 'None'. It is
commonly used to handle cases where a value may or
may not be present
*/

fn find_even(number: i32) -> Option<i32> {
    if number % 2 == 0 {
        Some(number)
    } else {
        None
    }
}

//Lesson 7: Collections and Ownership

// 7.1 Collections
/*
Rust has several collection types, including 'vec', 'String' and 'Hashmap'

*/

//7.1.1 Vec
//A vec is a growable, heap-allocated array

fn vec_fn(){
    let mut numbers = vec![1,2,3];

    numbers.push(4);
    numbers.push(5);

    for number in numbers {
        println!("Number: {}", number);
    }

}

//7.1.2 String
// A String is a growable, heap-allocated string

fn string_fn(){
    let mut message = String::from("Hello, ");
    message.push_str("World!");

    println!("{}", message);
}


//Ownership
/*
Rust enforces strict ownership rules to ensure memory safety
without a garbage collector

- Each value has a single owner
- when the owner goes out of scope, the value is dropped
- values can be moved or borrowed
*/

// fn move_fn() {
//     let s1 = String::from("hello");
//     let s2 = s1; //s1 is moved to s2 and s1 is no longer valid

//     println!("s1: {}", s1);//this would cause an error
//     println!("s2: {}", s2); //this works
// }

//Borrowing allows temporary access to a value without taking ownership

// -borrowing can be either mutable or immutable
// - you can have multiple immutable borrows or a single 
//mutable borrow at a time

fn borrow_fn() {
    let mut s = String::from("hello");

    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len );

    change(&mut s);
    println!("The modified string is: {}", s);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
