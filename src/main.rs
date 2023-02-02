#![allow(dead_code, unused_imports)]

fn main() {
    println!("Say something nice! （＾ω＾）")
}

#[doc = r#"**Sum of Cubes**

Write a function that takes a positive integer n, sums all the cubed values from 1 to n (inclusive), and returns that sum.


Assume that the input n will always be a positive integer.



I misundertsood the task, but it worked anyway. :-(


My solution: (1..=n).fold(0, | sum, x | sum + x).pow(2)


Stola' with pride: (1..=n).map(| x | x.pow(3)).sum()


<https://www.codewars.com/kata/59a8570b570190d313000037/train/rust>"#]
fn sum_cubes(n: u32) -> u32 {
    (1..=n).map(| x | x.pow(3)).sum()
}
#[test]
fn test_sum_cubes() {
    assert_eq!(sum_cubes(2), 9);
    assert_eq!(sum_cubes(3), 36);
}

/// Keep Hydrated!
///
/// Nathan loves cycling.
///
///Because Nathan knows it is important to stay hydrated, he drinks 0.5 litres of water per hour of cycling.
///
///You get given the time in hours and you need to return the number of litres Nathan will drink, rounded to the smallest value.
///
////// <https://www.codewars.com/kata/582cb0224e56e068d800003c/train/rust>
fn litres(time: f64) -> i32 {
    ( time * 0.5 ) as i32
}
#[test]
fn test_litres() {
    assert_eq!(litres(3.0), 1);
    assert_eq!(litres(6.7), 3);
    assert_eq!(litres(11.8), 5);
}

/// Simple multiplication
///
/// This kata is about multiplying a given number by eight if it is an even number and by nine otherwise.
///
/// <https://www.codewars.com/kata/583710ccaa6717322c000105/train/rust>
fn simple_multiplication(number: u8) -> u8 {
    if number % 2 == 0 { number * 8 }
    else { number * 9 }
}
#[test]
fn test_simple_multiplication() {
    assert_eq!(simple_multiplication(8), 64);
    assert_eq!(simple_multiplication(9), 81);
}

/// <https://www.codewars.com/kata/551b4501ac0447318f0009cd>
fn boolean_to_string(b: bool) -> String {
    b.to_string()
    // if b {return "true".to_string()}
    // else {return "false".to_string()}
}

/// <https://www.codewars.com/kata/514b92a657cdc65150000006/solutions/rust>
fn multiplies_of_3_or_5(num: i32) -> i32 {
    (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
