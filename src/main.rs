//Example 1
//How to print simple text: 
// fn main() { //main is important, can be changed, same with fn
// println!("Hello World!");
// }

//Example 2
//How does let work

// fn main(){
//     let x = 4; //let assigns the variable
//     //x = 5; this would fail, let makes variable immutable
//     println!("x is: {}", x);
//     let x: i32 = 9; //rewrites what x is
//     println!("x is: {}", x); //will print the new x, old x is forgotten

//     let mut y = 2; //mut makes the variable mutable, still cannot alter type
//     y = y + 6; //no need for let to rewrite the variable
//     println!("y is: {}", y);
//     { //curly braces within curly braces creates an inner step this is shadowing
//         let y = "changes in this inner layer do not effect the variable outside";
//         println!("y is: {}", y); //prints the inner y
//     }
//     println!("y is: {}", y); //outer y is not effected by the changes
// }

//Example 3 

//Define a function to calculate the factorial of a number.
// fn fact(n: u64) -> u64 {
//     // Base case: If n is 0 or 1, the factorial is 1.
//     if n == 0 || n == 1 {
//         1
//     } else {
//         // Recursive case: Calculate factorial by calling the function recursively.
//         n * fact(n - 1)
//     }
// } 

// fn main() {
//     // Define the number for which we want to calculate the factorial.
//     let num: u64 = 5; 

//     // Call the factorial function and store the result.
//     let result = fact(num); 

//     // Print the result to the console.
//     println!("Factorial of {} is: {}", num, result);
// }

//Example 4
// how do constants work

// fn main() {
//     const MONTHS_IN_A_YEAR: u32 = 12; //const makes this immutable 
//     //const MONTHS_IN_A_YEAR: u32 = 52; this will fail as a const cannot be changes
//     println!("There are {} months in a year", MONTHS_IN_A_YEAR);
// }

//Example 5
// tuples and things like that

// fn main() { //This is how tuples look and can be printed easily
//     let mut testtuple: (i32, char, bool) = (32,'s',false); //mut works the same as before
//     println!("The first item in my tuple {}, third item in my tuple {}, and second item in my tuple {}",
//     testtuple.0, testtuple.2, testtuple.1);
//     println!("my whole tuple is ({}, {}, {})", testtuple.0, testtuple.1, testtuple.2);
//     testtuple.0 = 45; //alter at the postion you want, same rules apply as before
//     println!("position 0 in tuple is now {}", testtuple.0);
// }

//Example 6
//Arrays

// fn main(){ //array only contains values of the same type
//     let array = [33,5,76,19,0,22]; //[] brackets for array, must have the exact number of values defined
//     println!("check out position 3 and 2 in my array: pos 3 = {}, pos 2 = {}", array[3], array[2]) //array position in [] brackets
// }

//Example 7
//crates and standard library

// use std::io;

// fn main(){
//     println!("ENTER A MESSAGE"); //call to action
//     let mut input = String::new(); //creates user input that is mutable

//     io::stdin().read_line(&mut input).expect("failed to read line"); //collects the input + error handling
//     println!("new input {}", input); //prints the input
// }


//Example 8
//arithmetic and type conversion
//numbers need to be the exact same type to do arithmetic

// fn main() {
//     let x = 100 as i8; //both numbers are set as different types
//     let y = 50 as i32;

//     let z = x / y as i8; //here they are converted together to the same type
//     let a = x as i64  * y as i64; //each number needs to be converted
//     println!("{}", z);
//     println!("{}", a)
// }

//number input
// use std::io;

// fn main(){
//     let mut input = String::new(); //similar to the previous input commands
//     io::stdin().read_line(&mut input).expect("read line");

//     let int_input: i64 = input.trim().parse().unwrap(); //trims the message, only accepts numbers

//     println!("{}", int_input + 5);
// }

//Example 9
//conditions

// fn main(){ //cond creates a condition True/False
//     let cond = 2 < 3;
//     let cond2 = false || !cond;
//     println!("{}", cond2);
// }
//&& is and
//|| is or
// ! is not

//if else statements choose your won adverture test

// fn main(){
//     let direction = "Down";

//     if direction == "North" {
//         println!("you walked north");
//         //another if/else look would start here
//     } else if direction == "East"{
//         println!("you walked east");
//     } else if direction == "South" {
//         println!("you walked south");
//     } else if direction == "West"{
//         println!("you walked west");
//     } else if direction == "Down" {
//         println!("you squatted down");
//     } else {
//         println!("no valid direction chosen");
//     }
// }

//functions

fn main(){
    println!("hello");
    test(); //calls the test function
    add_num(7, 22);
    sub_num(7, 22);
    mult_num(7, 22);
    div_num(21, 21);
}

fn test() { //creates a function called test that just prints
    println!("test has been called")
}

fn add_num(x: i32, y: i32) { //creates an addition function, can work with any symbol
    println!("the sum is: {}", x + y)
}

fn sub_num(x: i32, y: i32) { //creates an subtract function
    println!("the sum is: {}", x - y)
}

fn mult_num(x: i32, y: i32) { //creates an multiplication function
    println!("the sum is: {}", x * y)
}

fn div_num(x: i32, y: i32) { //creates an division function
    println!("the sum is: {}", x / y)
}


