/// A simple test function that demonstrates library functionality
pub fn test_function() -> String {
    "Hello from the hanseatic library!".to_string()
}

/// A utility function for demonstration
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// A function to check if a number is even
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Cities module containing Hanseatic League cities
pub mod cities;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(3));
        assert!(is_even(0));
    }

    #[test]
    fn test_test_function() {
        assert_eq!(test_function(), "Hello from the hanseatic library!");
    }
} 