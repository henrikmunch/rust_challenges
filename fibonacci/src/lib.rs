// Import HashMap data structure
use std::collections::HashMap;

// Function fib_memo(n, memo), where 'memo' is a mutable reference to HashMap
pub fn fib_memo(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    // If 'n' is inside 'memo': return 'result' (to save on computation time)
    // The 'Some' type: returns 'None' if there is no value in 'memo.get(&n)'
    if let Some(&result) = memo.get(&n) {
	return result;
    }

    // If 'n' is *not* inside 'memo': compute 'result' via recursion
    let result = match n {
	0 => 0,
	1 => 1,
	_ => fib_memo(n-1, memo) + fib_memo(n-2, memo),
    };

    // This is the memoization step
    // Insert 'result' into the 'memo' HashMap with key 'n' (key/value pair like in python dicts)
    memo.insert(n, result);
    // Output 'result'
    result
}

// Main Fibonaci function fib(n) that calls fib_memo(n, memo)
pub fn fib(n: usize) -> usize {
    // Initialize the 'memo' HashMap for memoization
    let mut memo: HashMap<usize, usize> = HashMap::new();
    fib_memo(n, &mut memo)
}
