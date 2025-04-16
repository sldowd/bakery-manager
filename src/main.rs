mod db;
mod models;
mod cli;

use db::{connect, get_all_inventory, get_recipe_collection, init_db, seed_inventory,
    seed_recipe_ingredients, seed_recipes, seed_transactions, reset_database};
use cli::show_main_menu;


fn main() {
    let conn = connect().expect("❌ Failed to connect to DB");
    init_db(&conn).expect("❌ Failed to initialize DB");
    if db::should_seed(&conn) {
        println!("🛠 Deleting and reseeding database...");
        // delete data before reseeding
        reset_database(&conn).expect("❌ Failed to reset database");
        // see database tables
        seed_inventory(&conn).expect("❌ Failed to seed inventory");
        seed_recipes(&conn).expect("Failed to seed recipes");
        seed_recipe_ingredients(&conn).expect("Failed to seed recipe ingredients");
        seed_transactions(&conn).expect("Faild to seed transactions");
    }

    loop {
        show_main_menu(&conn);
    }

}
