use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Recipe {
    pub recipe_id: i32,
    pub recipe_name: String,
    pub preparation: String,
}

pub struct Ingredient {
    pub ingredient_id: i32,
    pub substance_name: String,
    pub substance_quantity: f32,
    pub recipe_id: i32,
}

pub struct Substance {
    pub substance_name: String,
    pub units: String,
}

pub struct RecipeComplete {
    pub recipe: Recipe,
    pub ingredients: Vec<Ingredient>
}

fn to_opt<T,R>(res: Result<T,R>) -> Option<T>
    where R: std::fmt::Display {
        match res {
            Ok(v) => Some(v),
            Err(e) => {
                println!("Error {e}");
                None
            }
        }
}
pub struct Database {
    _pool: Pool,
    conn: PooledConn
}


impl Database {
    pub fn new() -> Option<Database> {
        
        let url = "mysql://username:password@localhost:3306/";      // username and password changed for git

        if let Some(pool) = to_opt(Pool::new(url)) {
            if let Some(mut conn) = to_opt(pool.get_conn()) {
                to_opt(conn.query_drop(
                    "CREATE DATABASE IF NOT EXISTS Recipes"))?;
                to_opt(conn.query_drop("USE Recipes"))?;
                Some(Database{_pool:pool, conn})
            }
            else { None }
        }
        else { None }
    }

    pub fn drop_tables(&mut self) -> Option<()> {
        to_opt(self.conn.query_drop(
            "DROP TABLE IF EXISTS Recipes"))?;
        to_opt(self.conn.query_drop(
            "DROP TABLE IF EXISTS Ingredients"))?;
        to_opt(self.conn.query_drop(
            "DROP TABLE IF EXISTS Substances"))?;
        Some(())
    }

    pub fn create_databases(&mut self, drop_first: bool) -> Option<()> {

        let drop = if drop_first {""} else {"IF NOT EXISTS"};
        if drop_first {
            self.drop_tables();
        }

        let query = format!("CREATE TABLE {} Recipes (
            `RecipeId` INT UNSIGNED NOT NULL AUTO_INCREMENT,
            `RecipeName` VARCHAR(50) NOT NULL,
            `Preparation` VARCHAR(500) NOT NULL,
            PRIMARY KEY (`RecipeId`)
            )", drop);
        to_opt(self.conn.query_drop(query))?;

        let query = format!("CREATE TABLE {} Ingredients (
            `IngredientId` INT NOT NULL AUTO_INCREMENT,
            `SubstanceName` VARCHAR(50) NOT NULL,
            `SubstanceQuantity` FLOAT NOT NULL,
            `RecipeId` INT NOT NULL,
            PRIMARY KEY (`IngredientId`)
            )", drop);
        to_opt(self.conn.query_drop(query))?;

        let query = format!("CREATE TABLE {} Substances (
            `SubstanceName` VARCHAR(50) NOT NULL,
            `Units` VARCHAR(50) NULL,
            PRIMARY KEY (`SubstanceName`)
            )", drop);
        to_opt(self.conn.query_drop(query))?;

        Some(())
    }

    pub fn get_substance(&mut self, name: &str) -> Option<Substance> {

        let query = format!("SELECT * FROM Substances Where SubstanceName = '{}'", name);
        match self.conn.query(query) {
            Ok(v) => {
                if v.is_empty() {
                    None
                }
                else {
                    let (a, b) : &(String, String) = &v[0];
                    Some(Substance{substance_name: a.clone(), units: b.clone()})
                }
            }
            Err(e) => {
                println!("get_substance: Error {e}");
                None
            }
        }
    }

    pub fn get_units(&mut self, name: &str) -> String {
        match self.get_substance(name) {
            Some(substance) => substance.units,
            None => String::new()
        }
    }

    pub fn add_substance(&mut self, subst: Substance) -> bool {

        let query = format!(
            "INSERT INTO Substances (SubstanceName,
            Units) VALUES ('{}', '{}')",
            subst.substance_name, subst.units);
        match self.conn.query_drop(query) {
            Ok(_) => true,
            Err(e) => {
                println!("add_substance: Error {e}");
                false
            }
        }
    }

    pub fn delete_substance(&mut self, substance_name: &str) -> bool {

        match self.conn.query_drop(format!(
            "DELETE FROM Substances WHERE SubstanceName = '{}'",
            substance_name)) {
                Ok(()) => true,
                Err(e) => {
                    println!("delete_substance: Error {e}");
                    false
                }
        }
    }

    pub fn get_ingredients(&mut self, id: i32) -> Option<Ingredient> {

        let query = format!(
            "SELECT * FROM Ingredients WHERE IngredientId = {}", id);
        
        match self.conn.query::<(i32, String, f32, i32), &str> (&query) {
            Ok(v) => {
                if v.is_empty() {
                    None
                }
                else {
                    let (id, subst, qty, rid)
                        : &(i32, String, f32, i32)
                        = &v[0];
                    Some(Ingredient {
                        ingredient_id: *id,
                        substance_name: subst.clone(),
                        substance_quantity: *qty,
                        recipe_id: *rid})
                }
            }
            Err(e) => {
                println!("get_ingredient: Error {e}");
                None
            }
        }
    }

    pub fn add_ingredient(&mut self, ingrnt: Ingredient) -> Option<i32> {

        match self.conn.query_drop(format!(
            "INSERT INTO Ingredients(
                SubstanceName,
                SubstanceQuantity,
                RecipeId)
                VALUES ('{}', '{}', '{}')",
            ingrnt.substance_name,
            ingrnt.substance_quantity,
            ingrnt.recipe_id)) {
                
                Ok(_) => {
                    let v = self.conn.last_insert_id();
                    Some(v.try_into().unwrap())
                },
                Err(e) => {
                    println!("add_ingredient: Error {e}");
                    None
                }
        }
    }

    pub fn delete_ingredient(&mut self, recipe_id: i32, 
        substance_name: &str) -> bool {
            
        match self.conn.query_drop(format!(
            "DELETE FROM Ingredients WHERE
            RecipeId = {} AND SubstanceName = '{}'",
            recipe_id, substance_name)) {

                Ok(_) => true,
                Err(e) => {
                    println!("delete_ingredient: Error {e}");
                    false
                }
        }
    }

    pub fn get_ingredients_in(&mut self, recipe_id: i32)
        -> Option<Vec<Ingredient>> {

        let query = format!("SELECT * FROM Ingredients
            WHERE RecipeId = {}", recipe_id);

        match self.conn.query::<(i32, String, f32, i32), &str>
            (&query) {

            Ok(v) => {
                let mut ingrnt: Vec<Ingredient> = Vec::new();

                for (id, subst, qty, rid) in v {
                    ingrnt.push(Ingredient { 
                        ingredient_id: id,
                        substance_name: subst.clone(),
                        substance_quantity: qty,
                        recipe_id: rid});
                }
                Some(ingrnt)
            }
            Err(e) => {
                println!("get_ingredient_in: Error {e}");
                None
            }
        }
    }

    fn get_recipe_with_condition(&mut self, with: &str)
        -> Option<Recipe> {
        
        let query = format!("SELECT * FROM Recipes {}", with);

        match self.conn.query(&query) {
            Ok(v) => {
                if v.is_empty() {
                    None
                }
                else {
                    let (id, name, prep) :
                        &(i32, String, String)
                        = &v[0];
                    Some(Recipe {
                        recipe_id: *id,
                        recipe_name: name.clone(),
                        preparation: prep.clone()})
                }
            }
            Err(e) => {
                println!("get_recipe: Error {e}");
                None
            }
        }
    }
        
    pub fn get_recipe_by_id(&mut self, id: i32)
        -> Option<Recipe> {
        
        let condition = format!("WHERE RecipeId = {}", id);

        self.get_recipe_with_condition(&condition)
    }

    pub fn get_recipe_by_name(&mut self, name: &str)
        -> Option<Recipe> {

        let condition = format!("WHERE RecipeName = '{}'", name);

        self.get_recipe_with_condition(&condition)
    }

    pub fn add_recipe(&mut self, recipe: Recipe)
        -> Option<i32> {
        
        match self.conn.query_drop(format!(
            "INSERT INTO Recipes (RecipeName, Preparation)
                VALUES ('{}', '{}')",
            recipe.recipe_name,
            recipe.preparation)) {

            Ok(_) => {
                let v = self.conn.last_insert_id();
                Some(v.try_into().unwrap())
            },
            Err(e) => {
                println!("add_recipe: Error {e}");
                None
            }
        }
    }

    pub fn delete_recipe(&mut self, recipe_id: i32) -> bool {

        match self.conn.query_drop(format!(
            "DELETE FROM Recipes WHERE RecipeId = {}", recipe_id)) {
            
            Ok(()) => match self.conn.query_drop(format!(
                "DELETE FROM Ingredients WHERE RecipeId = {}", recipe_id)) {

                    Ok(_) => true,
                    Err(e) => {
                        println!("delete_recipe(ingredients): Error {e}");
                        false
                    }
            },
            Err(e) => {
                println!("delete_recipe(recipe): Error {e}");
                false
            }
        }
    }

    pub fn get_recipe_complete(&mut self, recipe_name: &str)
        -> Option<RecipeComplete> {

        if let Some(recipe) = 
            self.get_recipe_by_name(recipe_name) {

            if let Some(ingredients) =
                self.get_ingredients_in(recipe.recipe_id) {
                
                Some(RecipeComplete{recipe, ingredients})
            }
            else {
                Some(RecipeComplete{recipe, ingredients: Vec::new()})
            }
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod test;

