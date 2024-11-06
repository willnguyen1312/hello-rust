// Simple recursive version
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

// Optimized recursive version using memoization
use std::collections::HashMap;

fn fibonacci_memoized(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    // Check if we've already calculated this value
    if let Some(&val) = memo.get(&n) {
        return val;
    }

    // Calculate new value and store in memo
    let val = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo),
    };

    memo.insert(n, val);
    val
}

fn main() {
    // Example usage of simple recursive version
    let n = 10;
    println!(
        "Simple recursive: The {}th Fibonacci number is: {}",
        n,
        fibonacci_recursive(n)
    );

    // Example usage of memoized version
    let mut memo = HashMap::new();
    println!(
        "Memoized recursive: The {}th Fibonacci number is: {}",
        n,
        fibonacci_memoized(n, &mut memo)
    );
}
