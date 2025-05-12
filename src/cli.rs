// src/cli.rs
use crate::db::{add_inventory_item, add_transaction, calculate_recipe_cost, deduct_recipe_from_inventory, filter_by_date, 
    get_all_inventory, get_ingredients_for_recipe, get_recipe_collection, read_transactions, transaction_filter, 
    update_inventory_cost, update_inventory_quantity, update_msrp_for_recipe, write_csv_transaction_report, reset_database,
    run_integrity_check, vacuum_database
};
use rusqlite::Connection;
use std::io::{self, Write};
use chrono::Local;
use std::fs;
use std::path::Path;
use std::env;

// CLI Helper Functions
// Pauses app and waits for user input
pub fn wait_for_enter() {
    let mut dummy_input = String::new();
    println!("\nPress Enter to return to the Main Menu...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dummy_input).unwrap();
}

// Backup database utility function
pub fn backup_database() {
    let source_path = Path::new("bakery.db");
    if !source_path.exists() {
        println!("❌ Error: bakery.db not found. No backup created.");
        return;
    }

    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_filename = format!("bakery_backup_{}.db", timestamp);

    if let Err(e) = fs::copy(source_path, &backup_filename) {
        println!("❌ Error copying database: {}", e);
    } else {
        println!("✅ Database backed up successfully to: {}", backup_filename);
    }
    wait_for_enter();
}

// Handle database reset
pub fn handle_database_reset(conn: &Connection) {
    println!("⚠️  This will delete ALL inventory, recipes, and transactions.");
            println!("Type 'YES' to confirm: ");
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();

            if confirm.trim() == "YES" {
                match reset_database(conn) {
                    Ok(_) => println!("✅ Database has been reset."),
                    Err(e) => println!("❌ Failed to reset database: {}", e),
                }
            } else {
                println!("❌ Reset cancelled.");
            }
            // Pause app and wait for user input
            wait_for_enter();
}
// View system info
pub fn view_system_info() {
    println!("📋 System Info:");

    // Current working directory
    match env::current_dir() {
        Ok(path) => println!("📁 Current Directory: {}", path.display()),
        Err(e) => println!("❌ Could not get current directory: {}", e),
    }

    // Does the database exist?
    if fs::metadata("bakery.db").is_ok() {
        println!("✅ bakery.db found.");
    } else {
        println!("❌ bakery.db not found.");
    }

    // Crude OS guess
    println!("🖥️  OS: {}", std::env::consts::OS);

    wait_for_enter();
}

// Handle data integrity check
pub fn handle_data_integrity_check(conn: &Connection) {
    println!("🔍 Running Data Integrity Check...");

    match run_integrity_check(conn) {
        Ok(issues) => {
            if issues.is_empty() {
                println!("✅ All integrity checks passed! No issues found.");
            } else {
                for issue in issues {
                    println!("❌ {}", issue);
                }
            }
        }
        Err(e) => {
            println!("❌ Error during integrity check: {}", e);
        }
    }

    wait_for_enter();
}

// Handle database vacuum
pub fn handle_vacuum(conn: &Connection) {

    println!("🧹 Compacting database...");

    match vacuum_database(conn) {
        Ok(_) => println!("✅ Database compacted successfully."),
        Err(e) => println!("❌ Failed to compact database: {}", e),
    }

    wait_for_enter();
}

// Helper function to calculate baker's percentages
fn calculate_bakers_percentages(ingredients: &Vec<(String, f32, String)>) {
    // Step 1: Find the total flour weight
    let mut total_flour_weight: f32 = 0.0;
    
    for (name, qty, unit) in ingredients {
        // Make sure we're only counting ingredients with weight units
        if unit.to_lowercase() == "g" || 
           unit.to_lowercase() == "grams" || 
           unit.to_lowercase() == "kg" {
            
            // Convert to grams if needed
            let quantity_in_grams = if unit.to_lowercase() == "kg" {
                qty * 1000.0
            } else {
                *qty
            };
            
            // Check if the ingredient name contains "flour" (case-insensitive)
            if name.to_lowercase().contains("flour") {
                total_flour_weight += quantity_in_grams;
            }
        }
    }
    // Display the ingredients
    println!("\nIngredients:");
    for (name, qty, unit) in ingredients {
        println!("- {} {} {}", qty, unit, name);
    }
    // If no flour was found, we can't calculate baker's percentages
    if total_flour_weight == 0.0 {
        println!("\n⚠️ No flour found in the recipe. Cannot calculate baker's percentages.");
        return;
    }
    
    // Step 2: Calculate and display the baker's percentages
    println!("\nBaker's Percentages:");
    println!("---------------------");
    
    // First, print the flour entries (should total 100%)
    let mut flour_percentage_total: f32 = 0.0;
    println!("Flour Components:");
    
    for (name, qty, unit) in ingredients {
        // Check if it's a weight unit
        if unit.to_lowercase() == "g" || 
           unit.to_lowercase() == "grams" || 
           unit.to_lowercase() == "kg" {
            
            // Convert to grams if needed
            let quantity_in_grams = if unit.to_lowercase() == "kg" {
                qty * 1000.0
            } else {
                *qty
            };
            
            if name.to_lowercase().contains("flour") {
                let percentage = (quantity_in_grams / total_flour_weight) * 100.0;
                flour_percentage_total += percentage;
                println!("- {}: {:.1}%", name, percentage);
            }
        }
    }
    
    println!("Total Flour: {:.1}% ({:.1} grams)", flour_percentage_total, total_flour_weight);
    
    // Then print all other ingredients
    println!("\nOther Ingredients:");
    for (name, qty, unit) in ingredients {
        if unit.to_lowercase() == "g" || 
           unit.to_lowercase() == "grams" || 
           unit.to_lowercase() == "kg" {
            
            // Convert to grams if needed
            let quantity_in_grams = if unit.to_lowercase() == "kg" {
                qty * 1000.0
            } else {
                *qty
            };
            
            if !name.to_lowercase().contains("flour") {
                let percentage = (quantity_in_grams / total_flour_weight) * 100.0;
                println!("- {}: {:.1}%", name, percentage);
                
                // Add special labels for common ingredients
                if name.to_lowercase().contains("water") {
                    println!("  (Hydration: {:.1}%)", percentage);
                } else if name.to_lowercase().contains("salt") {
                    println!("  (Salt percentage: {:.1}%)", percentage);
                } else if name.to_lowercase().contains("yeast") && percentage < 2.0 {
                    println!("  (Yeast percentage: {:.2}%)", percentage);
                } else if name.to_lowercase().contains("starter") || 
                          name.to_lowercase().contains("levain") || 
                          name.to_lowercase().contains("sourdough") {
                    println!("  (Starter percentage: {:.1}%)", percentage);
                }
            }
        } else {
            // For non-weight ingredients, just show them without percentages
            println!("- {}: {} {} (not included in baker's percentages)", 
                name, qty, unit);
        }
    }
    
    // Step 3: Detect and display special information about the recipe
    let mut hydration = 0.0;
    let mut salt_percentage = 0.0;
    let mut has_commercial_yeast = false;
    let mut has_sourdough = false;
    let mut starter_qty = 0.0;
    
    for (name, qty, unit) in ingredients {
        // Check if it's a weight unit
        if unit.to_lowercase() == "g" || 
           unit.to_lowercase() == "grams" || 
           unit.to_lowercase() == "kg" {
            
            // Convert to grams if needed
            let quantity_in_grams = if unit.to_lowercase() == "kg" {
                qty * 1000.0
            } else {
                *qty
            };
            
            let percentage = (quantity_in_grams / total_flour_weight) * 100.0;
            
            if name.to_lowercase().contains("water") {
                hydration += percentage;
            } else if name.to_lowercase().contains("salt") {
                salt_percentage += percentage;
            } else if name.to_lowercase().contains("yeast") && 
                      !name.to_lowercase().contains("sourdough") {
                has_commercial_yeast = true;
            } else if name.to_lowercase().contains("starter") || 
                      name.to_lowercase().contains("levain") || 
                      name.to_lowercase().contains("sourdough") {
                has_sourdough = true;
                starter_qty = quantity_in_grams;
            }
        }
    }
    
    // Adjust hydration if sourdough starter is present (assuming 100% hydration starter)
    if has_sourdough && starter_qty > 0.0 {
        // Calculate effective hydration from starter
        // For 100% hydration starter, half is water and half is flour
        let starter_water = starter_qty / 2.0;
        
        // Add water from starter to total hydration calculation
        let starter_water_percentage = (starter_water / total_flour_weight) * 100.0;
        println!("\nStarter contribution to hydration: {:.1}%", starter_water_percentage);
        
        // Note: We don't need to adjust total_flour_weight here since
        // the flour in the starter was already counted in the total flour calculation
    }
    
    // Print a recipe summary
    println!("\nRecipe Summary:");
    println!("Total flour: {:.1} grams", total_flour_weight);
    if hydration > 0.0 {
        println!("Hydration: {:.1}%", hydration);
    }
    if salt_percentage > 0.0 {
        println!("Salt: {:.1}%", salt_percentage);
    }
    
    // Detect what kind of bread this is
    if has_commercial_yeast && has_sourdough {
        println!("Type: Hybrid (commercial yeast + sourdough)");
    } else if has_sourdough {
        println!("Type: Sourdough");
    } else if has_commercial_yeast {
        println!("Type: Commercial yeast");
    } else {
        println!("Type: Quick bread or unknown (no yeast/starter detected)");
    }
    
    // Baker's percentage analysis
    println!("\nBaker's Percentage Analysis:");
    if hydration >= 65.0 && hydration < 70.0 {
        println!("Standard hydration level for most bread types");
    } else if hydration >= 70.0 && hydration < 80.0 {
        println!("High hydration - suitable for ciabatta, focaccia, or rustic breads");
    } else if hydration >= 80.0 {
        println!("Very high hydration - may be challenging to handle, consider stretch & folds");
    } else if hydration < 65.0 && hydration >= 60.0 {
        println!("Lower hydration - produces denser bread, easier to handle");
    } else if hydration < 60.0 {
        println!("Low hydration - typical for bagels, pretzels, or sandwich bread");
    }
    
    if salt_percentage > 0.0 {
        if salt_percentage < 1.8 {
            println!("Salt is below standard range (1.8-2.2%)");
        } else if salt_percentage > 2.2 {
            println!("Salt is above standard range (1.8-2.2%)");
        } else {
            println!("Salt is within standard range (1.8-2.2%)");
        }
    }
}

// Menu Functions
// Inventory Menu
pub fn handle_inventory_menu(conn: &Connection) {
    println!("🍞 Inventory Management");
    println!("1. View Inventory");
    println!("2. Add Inventory Item");
    println!("3. Update Inventory Item");
    println!("100. Exit");

    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        // View Inventory
        "1" => {
            let inventory = get_all_inventory(conn).expect("Error fetching inventory");
            println!("\n📦 Inventory:");
            for item in inventory {
                println!(
                    "{} - {}: {:.2} {} @ ${:.2}",
                    item.id, item.name, item.quantity, item.unit, item.cost_per_unit
                );
            }
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Add Inventory Item
        "2" => {
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
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Update Inventory Item
        "3" => {
            let mut input = String::new();

            println!("🍞 Update Inventory Item");

            // Fetch inventory
            let inventory = get_all_inventory(conn).expect("Error fetching inventory");
            println!("\n📦 Select an Item to update:");
            for item in &inventory {
                println!(
                    "{} - {}",
                    item.id, item.name
                );
            }
            
            // Identify inventory item to update
            print!("Enter item ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let inventory_item_id: i32 = input.trim().parse().unwrap_or(0);

            // Find selected item
            let selected_item = inventory.iter().find(|item| item.id == inventory_item_id);

            // Clear input before next read
            input.clear();
            
            // Identify which value to update
            println!("What would you like to update?\n1. Update Item Cost\n2. Update Item Quantity");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                // Update item cost
                "1" => {
                    if let Some(item) = selected_item {
                        // Output current cost per unit of selected item
                        println!("Current cost per unit for {}: ${:.2}", item.name, item.cost_per_unit);
                        input.clear();

                        println!("Enter updated item cost per unit: ");
                        
                        // Prompt user for updated cost per unit
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        let new_cost: f32 = input.trim().parse().unwrap_or(0.0);

                        // Call function to update item cost
                        let _update = update_inventory_cost(conn, inventory_item_id, new_cost);

                        // Confirm cost updated successfully to user
                        println!("✅ Successfully updated cost to ${:.2}!", new_cost);
                    } else {
                        println!("❌ Item not found!");
                    }

                    // Pause app and wait for user input
                    wait_for_enter();

                }
                // Update item quantity
                "2" => {
                    if let Some(item) = selected_item {
                        // Output current quatity of selected item 
                        println!("Current quantity for {}: ${:.2}", item.name, item.quantity);
                        input.clear();

                        // Prompt user for updated quantity
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                        let new_quantity: f32 = input.trim().parse().unwrap_or(0.0);

                        // Call function to update item quantity
                        let _update = update_inventory_quantity(conn, inventory_item_id, new_quantity);
                        
                        // Confirm quantity updated successfully to user
                        println!("✅ Successfully updated quantity to {:.2} units!", new_quantity);
                    } else {
                        println!("❌ Item not found!");
                    }

                    // Pause app and wait for user input
                    wait_for_enter();
                }
                &_ => {
                    println!("Error--Invalid option\n Returning to Main Menu...");
                }
            }
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Exit Inventory Menu
        "100" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        &_ => {
            println!("Error--Invalid option\n Returning to Main Menu...");
        }

    }
}

// Recipe Menu
pub fn handle_recipe_menu(conn: &Connection) {
    println!("📖 Recipe Management");
    println!("1. View Recipes");
    println!("2. View Recipe Ingredients");
    println!("3. Calculate Recipe Cost");
    println!("4. Deduct Recipe from Inventory");
    println!("5. Calculate Unit MSRP for Recipe");
    println!("6. Calculate Baker's Percentage for Recipe");
    println!("100. Exit");

    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        // View Recipes
        "1" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");
            println!("\n📖 Recipes:");
            for recipe in recipes {
                println!(
                    "ID: {} - {} MSRP: ${:?}\nCategory: {:#} \n(yields {})\nPrep time: {}\nBake time: {}\nTotal time: {}\n: \n{:#}\n",
                    recipe.id, recipe.name, recipe.msrp_per_unit, recipe.category, recipe.yield_quantity, recipe.prep_time.unwrap_or("N/A".to_string()), recipe.bake_time.unwrap_or("N/A".to_string()), recipe.total_time.unwrap_or("N/A".to_string()), recipe.instructions
                );
            }
            // Pause app and wait for user input
            wait_for_enter();
        }
        // View Recipe Ingredients
        "2" => {
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
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Calculate Recipe Cost
        "3" => {
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
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Deduct Recipe from Inventory
        "4" => {
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

            match result {
                Ok(_) => println!("✅ Recipe deducted from inventory."),
                Err(e) => println!("❌ Error deducting inventory: {}", e),
            }
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Calculate Unit MSRP for Recipe
        "5" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");

            println!("\nSelect a recipe to calculate MSRP:");
            for recipe in &recipes {
                println!("{}: {} (yield: {})", recipe.id, recipe.name, recipe.yield_quantity);
            }

            let mut input = String::new();
            print!("Enter recipe ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let recipe_id: i32 = input.trim().parse().unwrap_or(0);

            let base_cost = calculate_recipe_cost(conn, recipe_id).expect("Failed to calculate cost");
            if base_cost == 0.0 {
                println!("⚠️ No cost data found for this recipe.");
                return;
            }

            let recipe = get_recipe_collection(conn)
                .expect("Failed to fetch recipes")
                .into_iter()
                .find(|r| r.id == recipe_id)
                .expect("Recipe not found");

            let cost_per_unit = base_cost / recipe.yield_quantity as f32;

            println!("Cost per unit: ${:.2}", cost_per_unit);

            let mut markup_input = String::new();
            print!("Enter desired markup percentage (default 300): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut markup_input).unwrap();
            let markup: f32 = markup_input.trim().parse().unwrap_or(300.0);

            let msrp_per_unit = cost_per_unit * (markup / 100.0);

            println!("Calculated MSRP per unit: ${:.2}", msrp_per_unit);

            update_msrp_for_recipe(conn, recipe_id, msrp_per_unit).expect("Failed to update MSRP");

            println!("✅ MSRP saved for {}!", recipe.name);

            // Pause app and wait for user input
            wait_for_enter();
        }
        "6" => {
            let recipes = get_recipe_collection(conn).expect("Error fetching recipes");
        
            println!("\nSelect a recipe to calculate baker's percentages:");
            for recipe in &recipes {
                println!("{}: {}", recipe.id, recipe.name);
            }
        
            let mut input = String::new();
            print!("Enter recipe ID: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let recipe_id: i32 = input.trim().parse().unwrap_or(0);
        
            // Fetch the selected recipe to display its name
            let recipe = match recipes.iter().find(|r| r.id == recipe_id) {
                Some(r) => r,
                None => {
                    println!("⚠️ Recipe not found.");
                    wait_for_enter();
                    return;
                }
            };
        
            // Get the recipe ingredients with their details from the database
            let ingredients = get_ingredients_for_recipe(conn, recipe_id)
                .expect("Failed to load recipe ingredients");
            
            if ingredients.is_empty() {
                println!("⚠️ No ingredients found for that recipe.");
            } else {
                println!("\nRecipe: {}", recipe.name);
                println!("Yield: {} units", recipe.yield_quantity);
                
                // Display the ingredients
                println!("\nIngredients:");
                for (name, qty, unit) in &ingredients {
                    println!("- {} {} {}", qty, unit, name);
                }
                
                // Calculate and display baker's percentages
                calculate_bakers_percentages(&ingredients);
            }
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Exit Recipe Menu
        "100" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        &_ => {
            println!("Error--Invalid option\n Returning to Main Menu...");
        }
    }

}

// Transaction Menu
pub fn handle_transaction_menu(conn: &Connection) {
    println!("💰 Transaction Management");
    println!("1. Add Transaction");
    println!("2. View Transactions");
    println!("3. Filter Transactions");
    println!("4. Print CSV Transaction Report");
    println!("100. Exit");

    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        // Add Transaction
        "1" => {
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
            // Pause app and wait for user input
            wait_for_enter();
        }
        // View Transactions
        "2" => {
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
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Filter Transactions
        "3" => {
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
                    // Pause app and wait for user input
                    wait_for_enter();
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
                        // Pause app and wait for user input
                        wait_for_enter();
                    }
                &_ => {
                    println!("Error--input not accepted")
                }
                

            }
            // Pause app and wait for user input
            wait_for_enter();
        }
        // Print CSV Transaction Report
        "4" => {
            write_csv_transaction_report(conn).expect("Error: Failed to create report");

            println!("✅ Report created successfully");

            // Pause app and wait for user input
            wait_for_enter();
        }
        // Exit Transaction Menu
        "100" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        &_ => {
            println!("Error--Invalid option\n Returning to Main Menu...");
        }
    }
}

// Utilities Menu
pub fn handle_utilities_menu(conn: &Connection) {
    println!("🛠 Utilities");
    println!("1. Backup Database");
    println!("2. Reset Database");
    println!("3. View System Info");
    println!("4. Run Data Integrity Check");
    println!("5. Compact (VACUUM) Database");
    println!("100. Return to Main Menu");

    print!("Choose an option: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            backup_database();
        }
        "2" => {
            handle_database_reset(conn);
        }
        "3" => {
            view_system_info();
        }
        "4" => {
            handle_data_integrity_check(conn);
        }
        "5" => {
           handle_vacuum(conn);
        }
        "100" => {
            println!("Returning to Main Menu...");
            std::process::exit(0);
        }
        _ => println!("❌ Invalid option. Try again."),
    }
}

// function to display main CLI menu via main.rs
pub fn show_main_menu(conn: &Connection) {
    println!("\n🍞 Welcome to Bakery Manager CLI 🍞");
    
    println!("🍞 1. Inventory Management");
    println!("📖 2. Recipe Management");
    println!("💰 3. Transaction Management");
    println!("🛠 4. Utilities");
    println!("100. Exit");

    print!("Choose a category: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => handle_inventory_menu(conn),
        "2" => handle_recipe_menu(conn),
        "3" => handle_transaction_menu(conn),
        "4" => handle_utilities_menu(conn),
        // Exit Menu
        "100" => {
            println!("👋 Exiting. Goodbye!");
            std::process::exit(0);
        }
        _ => println!("❌ Invalid option. Try again."),
    }
    
}
