// src/cli.rs
use crate::db::{add_transaction, add_inventory_item, get_all_inventory, get_recipe_collection};
use rusqlite::Connection;
use time::Date;
use std::io::{self, Write};

pub fn show_main_menu(conn: &Connection) {
    println!("\n🍞 Welcome to Bakery Manager CLI 🍞");
    println!("1. View Inventory");
    println!("2. View Recipes");
    println!("3. Add Inventory Item");
    println!("4. Add Transaction");
    println!("10. Exit");
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
            let mut name = String::new();
            let mut unit = String::new();
            let mut quantity_str = String::new();
            let mut cost_str = String::new();

            println!("🍞 Add New Inventory Item");

            print!("Name: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut name).unwrap();

            print!("Unit (e.g. lbs, oz): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut unit).unwrap();

            print!("Quantity: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut quantity_str).unwrap();

            print!("Cost per unit: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut cost_str).unwrap();

            let quantity: f32 = quantity_str.trim().parse().unwrap_or(0.0);
            let cost: f32 = cost_str.trim().parse().unwrap_or(0.0);

            if let Err(e) = add_inventory_item(conn, name.trim(), unit.trim(), quantity, cost) {
                println!("❌ Failed to add item: {}", e);
            } else {
                println!(
                    "✅ Added {} ({} {}) at ${:.2}/unit",
                    name.trim(),
                    quantity,
                    unit.trim(),
                    cost
                );
            }

        }
        "4" => {
            let mut date = String::new();
            let mut transaction_type = String::new();
            let mut amount_str = String::new();
            let mut description = String::new();

            println!("💰 Add New Transaction");

            print!("Date (YYYY-MM-DD): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut date).unwrap();

            print!("Transaction type (sale/expense): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut transaction_type).unwrap();

            print!("Amount: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut amount_str).unwrap();

            print!("Description: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut description).unwrap();

            let amount: f32 = amount_str.trim().parse().unwrap_or(0.0);

            if let Err(e) = add_transaction(
                conn,
                date.trim(),
                transaction_type.trim(),
                amount,
                description.trim(),
            ) {
                println!("❌ Failed to add transaction: {}", e);
            } else {
                println!(
                    "✅ Logged ${:.2} {} on {} — {}",
                    amount,
                    transaction_type.trim(),
                    date.trim(),
                    description.trim()
                );
            }
        }

        "10" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        _ => println!("❌ Invalid option. Try again."),
    }
}
