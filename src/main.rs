mod db;
mod models;

use db::{connect, init_db, seed_inventory};

fn main() {
    println!("🍞 Welcome to the Bakery Manager CLI!");

    let flour = InventoryItem {
        id: 1,
        name: String::from("Flour"),
        unit: String::from("lbs"),
        quantity: 25.0,
        cost_per_unit: 0.50,
    };

    println!("Current inventory item: {:?}", flour);
}
