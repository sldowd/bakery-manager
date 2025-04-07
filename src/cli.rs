// src/cli.rs
use crate::db::{add_inventory_item, add_transaction, filter_by_date, get_all_inventory, get_recipe_collection, read_transactions, transaction_filter};
use rusqlite::Connection;
use time::Date;
use std::{io::{self, Write}, ptr::read};

pub fn show_main_menu(conn: &Connection) {
    println!("\n🍞 Welcome to Bakery Manager CLI 🍞");
    println!("1. View Inventory");
    println!("2. View Recipes");
    println!("3. Add Inventory Item");
    println!("4. Add Transaction");
    println!("5. View Transactions");
    println!("6. Filter Transactions");
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

        "5" => {
            let transactions = read_transactions(conn).expect("Error fetching transactions");
            println!("Transactions:");
            println!(
                "\n{:<4} | {:<12} | {:<10} | {:>8} | {}",
                "ID", "Date", "Type", "Amount", "Description"
            );
            println!("{}", "-".repeat(60));
            for transaction in transactions {
                println!(
                    "{:<4} | {:<12} | {:<10} | ${:>7.2} | {}",
                    transaction.id, transaction.date, transaction.transaction_type, transaction.amount, transaction.description
                )
            }
        }

        "6" => {
            // Filter by type or date
            print!("1. Filter by transaction type\n2. Filter by date\n");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => {
                    let mut query = String::new();
                    print!("Transaction type (sale/expense): ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut query).unwrap();

                    let transactions = transaction_filter(conn, query.trim()).expect("Error fetching transactions");
                    println!("Transactions:");
                    println!(
                        "\n{:<4} | {:<12} | {:<10} | {:>8} | {}",
                        "ID", "Date", "Type", "Amount", "Description"
                    );
                    println!("{}", "-".repeat(60));
                    for transaction in transactions {
                        println!(
                            "{:<4} | {:<12} | {:<10} | ${:>7.2} | {}",
                            transaction.id, transaction.date, transaction.transaction_type, transaction.amount, transaction.description
                        )
                    }
                }
                "2" => {
                        let mut date = String::new();
                        print!("Date (YYYY-MM-DD): ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut date).unwrap();
                        
                        let transactions = filter_by_date(conn, date.trim()).expect("Error fetching transactions");
                        println!("Transactions:");
                        println!(
                            "\n{:<4} | {:<12} | {:<10} | {:>8} | {}",
                            "ID", "Date", "Type", "Amount", "Description"
                        );
                        println!("{}", "-".repeat(60));
                        for transaction in transactions {
                            println!(
                                "{:<4} | {:<12} | {:<10} | ${:>7.2} | {}",
                                transaction.id, transaction.date, transaction.transaction_type, transaction.amount, transaction.description
                            )
                        }
                    }
                &_ => {
                    println!("Error--input not accepted")
                }
                

            }
        }
        "10" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        _ => println!("❌ Invalid option. Try again."),
    }
}
