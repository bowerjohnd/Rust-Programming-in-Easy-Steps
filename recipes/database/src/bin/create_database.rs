use database::Database;

fn main() {

    if let Some(mut database) = Database::new() {
        match database.create_databases(false) {
            Some(_) => println!("Databases created"),
            None => println!("Failed to create databases")
        }
    }
}