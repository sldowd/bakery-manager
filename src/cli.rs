// src/cli.rs
use crate::db::{get_all_inventory, get_recipe_collection};
use rusqlite::Connection;
use std::io::{self, Write};

pub fn show_main_menu(conn: &Connection) {
    println!("\n🍞 Welcome to Bakery Manager CLI 🍞");
    println!("1. View Inventory");
    println!("2. View Recipes");
    println!("3. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            let inventory = get_all_inventory(conn).expect("Error fetching inventory");
            println!("\n📦 Inventory:");
            for item in inventory {
                println!(
                    "- {}: {:.2} {} @ ${:.2}",
                    item.name, item.quantity, item.unit, item.cost_per_unit
                );
            }
        }
        "2" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");
            println!("\n📖 Recipes:");
            for recipe in recipes {
                println!(
                    "- {} (yields {}): {}",
                    recipe.name, recipe.yield_quantity, recipe.instructions
                );
            }
        }
        "3" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        _ => println!("❌ Invalid option. Try again."),
    }
}
