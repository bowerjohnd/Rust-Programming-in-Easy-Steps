use database::{Database, Ingredient, Recipe};
use prompt::{prompted, prompted_no_blank};

// insert the rest of the code here 

fn store_ingredient(database: &mut Database,
    substance_name: &str, substance_quantity: f32,
    substance_units: &str, recipe_id:i32) -> bool {

    match database.add_ingredient(
        Ingredient{ingredient_id:0,
            substance_name: substance_name.into(),
            substance_quantity, recipe_id}) {

        Some(ingredient_id_) => {
            println!("Added {substance_quantity} {} \
                of {substance_name} as \
                ingredient {ingredient_id_}",
                substance_units);
            true
        },
        None => false

    }
    
}

fn enter_ingredients(database: &mut Database, recipe_id:i32)
    -> bool {
    
    println!("Ingredients:");
    loop {
        if let Some(substance_name) =
            prompted::<String>("Substance") {

            if let Some(substance) =
                database.get_substance(&substance_name) {
                        
                let substance_quantity: f32 =
                        prompted_no_blank(&format! (
                        "Quantity ({})", substance.units));
                    if !store_ingredient(database,
                        &substance_name,
                        substance_quantity,
                        &substance.units,
                        recipe_id) {
                            
                            return false;
                    }
                    else {
                        println!("Unknown Substance");
                    }
            }
            else {
                return true;
            }
        }
    }
}  

fn main() {
    let mut database = Database::new()
        .expect("Failed to open databse");

    let recipe_name: String = prompted_no_blank("Recipe Name");
    let preparation: String = prompted_no_blank("Preparation");

    if let Some(recipe_id) =
        database.add_recipe(Recipe{recipe_id:0,
            recipe_name, preparation}) {

        if !enter_ingredients(&mut database, recipe_id) {
                    database.delete_recipe(recipe_id);
        }
    }
}