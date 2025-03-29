mod db;
mod models;

use db::{connect, get_all_inventory, init_db, seed_inventory, seed_recipes};

fn main() {
    let conn = connect().expect("❌ Failed to connect to DB");
    init_db(&conn).expect("❌ Failed to initialize DB");
    seed_inventory(&conn).expect("❌ Failed to seed inventory");
    seed_recipes(&conn).expect("Faild to seed recipes");

    // fetch inventory from database
    let inventory = get_all_inventory(&conn).expect("❌ Failed to fetch inventory");

    // print current inventory
    println!("📦 Current Inventory:");
    for item in inventory {
        println!(
            "- {}: {:.2} {} at ${:.2}/unit",
            item.name, item.quantity, item.unit, item.cost_per_unit
        );
    }
}
