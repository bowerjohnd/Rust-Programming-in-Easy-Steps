use mysql::*;
use mysql::prelude::*;

// Quick program to test connection to mysql database

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://username:password@localhost:3306/";      // username and password changed for git
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // Example query
    let result: Vec<String> = conn.query("show databases")?;
    for row in result {
        println!("Table: {}", row);
    }

    Ok(())
}
