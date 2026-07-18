//! A module with a single function in it to help us
//! pick a random string out of an array of
//! possible strings.

use rand::prelude::*;

/// Returns one of several strings at random.
/// The argument should be an array of strings.
/// This function will panic if given an empty array.
/// 
/// # Examples
/// ```
/// use hello_complex::choose::one_of;
/// let r = one_of(&["a","b","c"]);
/// assert!(r == "a" || r == "b" || r == "c");
/// ```

pub fn one_of(choices:&[&str]) -> String {
    let mut rng = rand::rng();
    let i = rng.random_range(..choices.len());
    String::from( choices[i])
}

#[test]
fn test_one_of() {
    let x = one_of(&["A","B","C","D"]);
    assert!(x.len() == 1);
}