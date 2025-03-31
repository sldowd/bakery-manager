mod db;
mod models;
mod cli;

use db::{connect, get_all_inventory, get_recipe_collection, init_db, seed_inventory, seed_recipes};
use cli::show_main_menu;


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

    // fetch recipes from database
    let recipes = get_recipe_collection(&conn).expect("❌ Failed to get recipe collection");

    println!("\n📖 Recipe Collection:");
    for recipe in recipes {
        println!(
            "- {} (yields {}): {}",
            recipe.name, recipe.yield_quantity, recipe.instructions
        );
    }

    loop {
        show_main_menu(&conn);
    }

}
