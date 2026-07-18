use std::env;
use database::{Database, Substance};

// Insert the rest of the code below here

fn store(database: &mut Database, record: csv::StringRecord) {

    if record.len() < 1 {
        return;
    }
    else {
        let substance: String = record[0].trim().into();
        let units = if record.len() < 2 { "" }
            else { &record[1].trim() };
        println!("Substance '{substance}' in units of '{units}'");

        if !database.add_substance(
            Substance{substance_name: substance.clone(),
                units: units.into()}) {
                    println!("{substance}: Failed to update database")
        }
    }
}

fn import(database: &mut Database, file: &str) {
    println!("Importing substances from {file}");
    match csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file) {

            Err(e) => println!("Failed to open {file}: {e}"),

            Ok(mut rdr) => {
                println!("Opened file and created reader");
                for result in rdr.records() {
                    // The iterator yields
                    // Result <StringRecrod, Error>,
                    // so we check the error here.
                    match result {
                        Ok(record) => {
                            store(database, record);
                        },
                        Err(e) => {
                            println!("Invalid CSV line: {e}")
                        }
                    }
                }
            }
        }
}


fn main() {
    if let Some(mut database) = Database::new() {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Usage: {} <csv-file>", args[0]);
        }
        else {
            import(&mut database, &args[1]);
        }
    }
    else {
        println!("Failed to open database")
    }
}
