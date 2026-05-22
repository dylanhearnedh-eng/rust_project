//Example 1
//How to print simple text: 
// fn main() { //main is important, can be changed, same with fn
// println!("Hello World!");
// }

//Example 2
// fn main() {  //main is essential here
//     let x = 4;
//     println!("x is: {}", x);
//     {
//         let x = "hello";
//         println!("x is: {}", x);
//     }
//     let x = x + 1;
//     println!("x is: {}", x);
// }

//Example 3 

//Define a function to calculate the factorial of a number.
fn fact(n: u64) -> u64 {
    // Base case: If n is 0 or 1, the factorial is 1.
    if n == 0 || n == 1 {
        1
    } else {
        // Recursive case: Calculate factorial by calling the function recursively.
        n * fact(n - 1)
    }
} 

fn main() {
    // Define the number for which we want to calculate the factorial.
    let num: u64 = 5; 

    // Call the factorial function and store the result.
    let result = fact(num); 

    // Print the result to the console.
    println!("Factorial of {} is: {}", num, result);
}



