use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let script = "name = game
    The screen is 1024 pixels square
    x = 1024
    y = 1024
    z = 1024
    The higher the gamma, the brighter the picture,
    but the lower the contrast
    gamma = 1
    We Start out ultra-cute
    cuteness = 9";

    let variables = HashMap::from(
        [("x", 1), ("y", 2), ("gamma", 3)]
    )
    ;

    let settings = script.split("\n")
        .filter_map(|x| x.split("=").next_tuple())
        .map( | (var, val) | {
            let var = var.trim();
            (var, variables.get(var), val.trim())
        })
        // Add more iterators here
    ;

    let var_table = settings.clone()
        .filter_map( | (_var, index, val) | match index {
            Some(s) => Some((s, val)),
            None => None
        });

    let err_table = settings    
        .filter_map( | (var, index, val) | match index {
            None => Some((var, val)),
            Some(_) => None
        });

// Used before settings.map iterator added.
    // for (var, val) in settings{
    //     println!("var[{var}] <- {val}");
    // }

// Not shown in book, found in free resources download examples from ineasysteps.com
    println!("\nVariable Table");
    for (var, val) in var_table {
        println!("var[{var}] <- {val}");
    }

    println!("\nError Table");
    for (var, val) in err_table {
        println!("var[{var}] <- {val}");   
    }

    println!("");
}
