use database::{Database};
use prompt::prompted_no_blank;

fn main() {
    let mut database = Database::new()
        .expect("Failed to open database");

    let recipe_name: String = prompted_no_blank("Recipe Name");

    match database.get_recipe_complete(&recipe_name) {
        None => {
            println!("There is no recipe called '{recipe_name})");
            return
        }
        Some(recipe) => {
            println!("{recipe_name}\n");
            for item in recipe.ingredients {
                let units = database.get_units(
                    &item.substance_name);
                println!("   {} {} {}",
                    item.substance_quantity,
                    units,
                    item.substance_name);
                }
                println!("\nPreparation:\n{}", recipe.recipe.preparation);
        }
    }
}

