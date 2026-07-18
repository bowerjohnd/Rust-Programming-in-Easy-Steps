use std::collections::HashMap;

#[derive (Eq, PartialEq, Hash)]
enum Objects {Chicken, Deeds, Bust}

fn main() {

    use Objects::{*};

    let names = HashMap::from([
        (Chicken, "a rubber chicken"),
        (Deeds, "the deeds to the Brooklyn Bridge"),
        (Bust, "a plaster bust of Shakespeare")
    ]);
    let values = HashMap::from([
        (Chicken, 3), (Deeds, 1), (Bust, 5)
    ]);

    let mut matchsticks = 8;
    let mut objects: Vec<Objects> = Vec::new();
    objects.push(Bust);
    objects.push(Chicken);

    println!("For a pot of {matchsticks}...");
    
    for item in objects {
        println!("The dealer offers {}", names[&item]);
        matchsticks -= values[&item]
    }
    println!("And {matchsticks} matchsticks");
}
