/// **Quarter of the year**
///
/// Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.
///
/// For example: month 2 (February), is part of the first quarter;
/// month 6 (June), is part of the second quarter;
/// and month 11 (November), is part of the fourth quarter.
///
/// Constraint: 1 <= month <= 12
///
/// <https://www.codewars.com/kata/5ce9c1000bab0b001134f5af>
fn quarter_of(month: u8) -> u8 {
    match month {
        1..=3 => 1,
        4..=6 => 2,
        7..=9 => 3,
        10..=12 => 4,
        _ => unreachable!("Wrong input.\nIt should be between 1 and 12.")
    }
}
#[test]
fn test_quarter_of() {
    assert_eq!(quarter_of(2), 1);
    assert_eq!(quarter_of(6), 2);
    assert_eq!(quarter_of(7), 3);
    assert_eq!(quarter_of(11), 4);
}

/// **Convert a Boolean to a String**
///
/// Implement a function which convert the given boolean value into its string
/// representation.
///
/// Note: Only valid inputs will be given.
///
/// <https://www.codewars.com/kata/551b4501ac0447318f0009cd>
fn boolean_to_string(b: bool) -> String {
    b.to_string()
    // if b {return "true".to_string()}
    // else {return "false".to_string()}
}
#[test]
fn test_boolean_to_string() {
    assert_eq!(boolean_to_string(true), "true".to_string());
    assert_eq!(boolean_to_string(false), "false".to_string());
}

/// **Simple multiplication**
///
/// This kata is about multiplying a given number by eight if it is
/// an even number and by nine otherwise.
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

/// **Keep Hydrated!**
///
/// Nathan loves cycling.
///
/// Because Nathan knows it is important to stay hydrated,
/// he drinks 0.5 litres of water per hour of cycling.
///
/// You get given the time in hours and you need to return
/// the number of litres Nathan will drink, rounded to the smallest value.
///
/// <https://www.codewars.com/kata/582cb0224e56e068d800003c/train/rust>
fn litres(time: f64) -> i32 {
    ( time * 0.5 ) as i32
}
#[test]
fn test_litres() {
    assert_eq!(litres(3.0), 1);
    assert_eq!(litres(6.7), 3);
    assert_eq!(litres(11.8), 5);
}
