use hanseatic::{test_function, add_numbers, is_even};

fn main() {
    println!("Hello, world!");
    
    // Call the test function from the library
    let test_result = test_function();
    println!("Test function result: {}", test_result);
    
    // Demonstrate other library functions
    let sum = add_numbers(15, 25);
    println!("15 + 25 = {}", sum);
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Even numbers from 1-10:");
    for num in numbers {
        if is_even(num) {
            println!("  {}", num);
        }
    }
}
