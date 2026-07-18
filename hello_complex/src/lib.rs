pub mod choose;
use choose::one_of;

pub fn addressee() -> String {
    let audiences = ["World", "Everyone", "Universe"];
    one_of(&audiences)
}

pub fn greeting() -> String {
    let salutations = ["Hello", "Hi", "Welcome", "Heya"];
    one_of(&salutations)
}
