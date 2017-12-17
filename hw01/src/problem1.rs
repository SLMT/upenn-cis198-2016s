
/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut result: i32 = 0;
    for i in slice {
        result += *i;
    }
    result
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vs {
        if !result.contains(v) {
            result.push(*v);
        }
    }
    result
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for v in vs {
        if pred(*v) {
            result.push(*v);
        }
    }
    result
}
