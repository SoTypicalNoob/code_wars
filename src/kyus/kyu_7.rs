/// **Sum of Cubes**
///
/// Write a function that takes a positive integer n,
/// sums all the cubed values from 1 to n (inclusive), and returns that sum.
///
/// Assume that the input n will always be a positive integer.
///
/// <https://www.codewars.com/kata/59a8570b570190d313000037>"
fn sum_cubes(n: u32) -> u32 {
    (1..=n).map(| x | x.pow(3)).sum()
}
#[test]
fn test_sum_cubes() {
    assert_eq!(sum_cubes(2), 9);
    assert_eq!(sum_cubes(3), 36);
}

/// **String ends with?**
///
/// Complete the solution so that it returns true if the first argument(string)
/// passed in ends with the 2nd argument (also a string).
///
/// <https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d>
fn string_ends_with(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}
#[test]
fn test_string_ends_with() {
    assert_eq!(string_ends_with("abc", "bc"), true);
    assert_eq!(string_ends_with("abc", "d"), false)
}
