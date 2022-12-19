mod database_sql;

fn main() {
    println!("Hello, world!");
    // Create a connection to the database:
    let db = database_sql::DatabaseSQL::new().unwrap();
    println!("Hashes: {:?}", db.count_hashes());
}
