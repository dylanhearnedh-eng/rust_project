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
//     { //curly braces within curly braces creates an inner step
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

fn main(){ //array only contains values of the same type
    let array = [33,5,76,19,0,22]; //[] brackets for array, must have the exact number of values defined
    println!("check out position 3 and 2 in my array: pos 3 = {}, pos 2 = {}", array[3], array[2]) //array position in [] brackets
}


