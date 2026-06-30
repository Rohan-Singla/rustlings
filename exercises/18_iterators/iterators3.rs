#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    // Check for division by zero
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    // Check for integer overflow: only i64::MIN / -1 would overflow
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    let div = a / b;
    let rem = a % b;
    if rem == 0 {
        Ok(div)
    } else {
        Err(DivisionError::NotDivisible)
    }
}

// Returns Ok with a list of numbers divided by 27, or the first encountered error
fn result_with_list() -> Result<[i64; 4], DivisionError> {
    let numbers = [27, 297, 38502, 81];
    // Attempt to divide each number, fail fast on error
    let mut arr = [0i64; 4];
    for (i, n) in numbers.iter().enumerate() {
        arr[i] = divide(*n, 27)?;
    }
    Ok(arr)
}

// Returns a vector of the results of dividing each number by 27.
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers.iter().map(|&n| divide(n, 27)).collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
