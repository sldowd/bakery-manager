// src/cli.rs
use crate::db::{add_inventory_item, add_transaction, calculate_recipe_cost, deduct_recipe_from_inventory,
    filter_by_date, get_all_inventory, get_ingredients_for_recipe, get_recipe_collection,
    read_transactions, transaction_filter, write_csv_transaction_report};
use rusqlite::Connection;
use std::io::{self, Write};

// function to displat CLI via main.rs
pub fn show_main_menu(conn: &Connection) {
    println!("\n🍞 Welcome to Bakery Manager CLI 🍞");
    println!("1. View Inventory");
    println!("2. View Recipes");
    println!("3. Add Inventory Item");
    println!("4. Add Transaction");
    println!("5. View Transactions");
    println!("6. Filter Transactions");
    println!("7. View Recipe Ingredients");
    println!("8. Calculate Recipe Cost");
    println!("9. Deduct Recipe from Inventory");
    println!("10. Print CSV Transaction Report");
    println!("100. Exit");
    println!("110. Debug");
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
                    "{} - {}: {:.2} {} @ ${:.2}",
                    item.id, item.name, item.quantity, item.unit, item.cost_per_unit
                );
            }
        }
        "2" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");
            println!("\n📖 Recipes:");
            for recipe in recipes {
                println!(
                    "ID: {} - {}\nCategory: {:#} \n(yields {}): \n{:#}\n",
                    recipe.id, recipe.name, recipe.category, recipe.yield_quantity, recipe.instructions
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
            let mut input = String::new();
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
        "7" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");

            println!("\nSelect a recipe to view ingredients:");
            for recipe in &recipes {
                println!("{}: {}", recipe.id, recipe.name);
            }
        
            let mut input = String::new();
            print!("Enter recipe ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let recipe_id: i32 = input.trim().parse().unwrap_or(0);
        
            let ingredients = get_ingredients_for_recipe(conn, recipe_id).expect("Failed to load ingredients");
            
            if ingredients.is_empty() {
                println!("⚠️ No ingredients found for that recipe.");
            } else {
                for (name, qty, unit) in &ingredients {
                    println!("- {} {} {}", qty, unit, name);
                }
            }
        }
        "8" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");

            println!("\nSelect a recipe to view ingredients:");
            for recipe in &recipes {
                println!("{}: {}", recipe.id, recipe.name);
            }

            let mut input = String::new();
            print!("Enter recipe ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let recipe_id: i32 = input.trim().parse().unwrap_or(0);

            let recipe_cost: f32 = calculate_recipe_cost(conn, recipe_id).expect("Failed to calculate cost");

            if recipe_cost == 0.0 {
                println!("Calulation failed or returned zero");
            } else {
                println!("Total recipe cost: ${:.2}", recipe_cost);
            }

        }
        "9" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");

            println!("\nSelect a recipe to view ingredients:");
            for recipe in &recipes {
                println!("{}: {}", recipe.id, recipe.name);
            }

            let mut input = String::new();
            print!("Enter recipe ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let recipe_id: i32 = input.trim().parse().unwrap_or(0);

            let result = deduct_recipe_from_inventory(conn, recipe_id);

            //println!("{:?}", result);

            match result {
                Ok(_) => println!("✅ Recipe deducted from inventory."),
                Err(e) => println!("❌ Error deducting inventory: {}", e),
            }
            
        }
        "10" => {
            write_csv_transaction_report(conn).expect("Error: Failed to create report");
        }
        "100" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        "110" => {
            let count: i32 = conn.query_row(
                "SELECT COUNT(*) FROM recipe_ingredients",
                [],
                |row| row.get(0),
            ).unwrap();
            
            println!("Rows in recipe_ingredients: {}", count);
        }
        _ => println!("❌ Invalid option. Try again."),
    }
}
