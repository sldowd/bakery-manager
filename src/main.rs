mod db;
mod models;

use db::{connect, init_db, seed_inventory};

fn main() {
    let conn = connect().expect("Failed to connect to SQLite");
    println!("✅ Connected to bakery.db");

    init_db(&conn).expect("Failed to initialize schema");
    println!("✅ Tables initialized");

    seed_inventory(&conn).expect("Failed to seed inventory");
}
