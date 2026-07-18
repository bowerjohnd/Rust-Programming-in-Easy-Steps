
use super::*;

#[test]
fn drop_tables() {
    let mut database = Database::new().unwrap();
    assert!(!database.drop_tables().is_none());
}

#[test]
fn create_and_drop() {
    let mut database = Database::new().unwrap();
    assert!(!database.create_databases(true).is_none());
    assert!(!database.create_databases(false).is_none());
    assert!(!database.drop_tables().is_none());
}

#[test]
fn insert_read_delete_substance() {

    let mut database = Database::new().unwrap();
    assert!(!database.create_databases(false).is_none());
    assert!(database.add_substance(
        Substance{substance_name: "foo".into(), units: "ml".into()}));
    assert!(database.get_units("foo") == "ml");
    assert!(database.get_units("bar") == "");
    if let Some(substance) = database.get_substance("foo") {
        assert!(substance.units == "ml");
    }
    else {
        panic!("Failed to retrieve added substance");
    }
    if let Some(_substance) = database.get_substance("bar") {
        panic!("Retrieval of invalid substance succeeded");
    }

    assert!(database.delete_substance("foo"));
    if let Some(_substance) = database.get_substance("foo") {
        panic!("Retrieval of deleted substance succeeded");
    }
}