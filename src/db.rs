// src/db.rs
use rusqlite::{Connection, Result, Row, params};
use crate::models::InventoryItem;
use crate::models::RecipeCollection;
use crate::models::Transaction;
use std::fs::File;
use std::io;
use csv::Writer;

pub fn connect() -> Result<Connection> {
    Connection::open("bakery.db")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS inventory (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            unit TEXT NOT NULL,
            quantity REAL NOT NULL,
            cost_per_unit REAL NOT NULL
        );

        CREATE TABLE IF NOT EXISTS recipes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            instructions TEXT NOT NULL,
            yield_quantity INTEGER NOT NULL,
            category TEXT NOT NULL,
            prep_time TEXT,
            bake_time TEXT,
            total_time TEXT
        );

        CREATE TABLE IF NOT EXISTS recipe_ingredients (
            recipe_id INTEGER,
            ingredient_id INTEGER,
            quantity_required REAL NOT NULL,
            FOREIGN KEY(recipe_id) REFERENCES recipes(id),
            FOREIGN KEY(ingredient_id) REFERENCES inventory(id)
        );

        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            transaction_type TEXT NOT NULL,
            amount REAL NOT NULL,
            description TEXT
        );
        "
    )?;
    conn.execute(
        "ALTER TABLE recipes ADD COLUMN msrp_per_unit REAL",
        [],
    ).ok();

    Ok(())    
    
}

// seed inventory
pub fn seed_inventory(conn: &Connection) -> Result<()> {
    let sample_inventory = vec![
        ("Flour, Bread", "kg", 15.8, 1.87),
        ("Sugar, Organic Granulated", "kg", 9.5, 1.98),
        ("Butter", "grams", 20.0, 3.25),
        ("Eggs", "each", 15.0, 1.20),
        ("Vanilla", "ml", 118.0, 0.09),
        ("Butter, Unsalted", "grams", 907.18, 0.0138),
        ("Sugar, Light Brown", "kg", 3.175, 2.20),
        ("Flour, Whole Wheat", "kg", 2.268, 2.55),
        ("Flour, Organic Rye", "kg", 1.361, 8.81),
        ("Salt, Maldon Flaked Sea", "grams", 567.0, 0.0123),
        ("Olive Oil, Organic Extra Virgin", "ml", 1998.0, 0.0075),
        ("Salt, Kosher", "grams", 1360.78, 0.0022),
        ("Flour, Unbleached All Purpose", "kg", 5.443, 1.98),
        ("Water", "grams", 10000.0, 0.0),
        ("Sourdough Starter", "grams", 1000.0, 0.00107),
        ("Raisins", "grams", 500.0, 0.0077),
        ("Cinnamon", "grams", 100.0, 0.033),
        ("Milk Powder, Nonfat", "grams", 623.7, 0.02403),
        ("Yeast, Instant", "grams", 113.4, 0.05811),
        ("Honey", "grams", 566.99, 0.02646),
        ("Vegetable Oil", "grams", 2721.55, 0.00202),
        ("Chocolate Chips, Dark", "grams", 283.0, 0.02823),
        ("Cocoa Powder", "grams", 226.8, 0.03082),
        ("Espresso Powder", "grams", 100.0, 0.09490),
        ("Confectioners' Sugar", "grams", 907.18, 0.00440),
        ("Sesame Seeds", "grams", 60.1, 0.04975),
        ("Orange Zest", "grams", 130.0, 0.00769),
        ("Rosemary", "grams", 100.0, 0.0),
        ("Cream Cheese", "grams", 170.0, 0.0),
        ("Sour Cream", "grams", 90.0, 0.0),
        ("Ricotta Cheese", "grams", 454.0, 0.0),
        ("Lemon Zest", "grams", 5.0, 0.0),
        ("Lemon Juice", "grams", 60.0, 0.0),
        ("Turbinado Sugar", "grams", 34.0, 0.0),
        ("Sparkling Sugar", "grams", 43.0, 0.0),
        ("Cream of Tartar", "grams", 10.0, 0.0),
        ("Potato Flour", "grams", 46.0, 0.0),
        ("Dill Pickle Juice", "grams", 170.0, 0.0),
        ("Caraway Seeds", "grams", 10.0, 0.0),
        ("Dill Seeds", "grams", 10.0, 0.0),
        ("Mustard Seeds", "grams", 10.0, 0.0),
        ("Walnuts", "grams", 113.0, 0.0),
        ("Dried Cranberries", "grams", 85.0, 0.0),
        ("Fiori di Sicilia", "ml", 5.0, 0.0),
        ("Vanilla Bean Paste", "ml", 5.0, 0.0),
        ("Pizza Flour, 00", "grams", 180.0, 0.0),
        ("Cake Flour", "grams", 180.0, 0.0),
        ("Unbleached Cake Flour", "grams", 120.0, 0.0),
        ("Egg Whites", "grams", 420.0, 0.0),
        ("Baking Powder", "grams", 10.0, 0.0),
        ("Pecans", "grams", 113.0, 0.0),
        ("Apricot Preserves", "grams", 64.0, 0.0),
        ("Apple", "grams", 113.0, 0.0),
        ("Cornstarch", "grams", 14.0, 0.0),
        ("Lime Juice", "grams", 5.0, 0.0),
        ("Orange Juice", "grams", 15.0, 0.0),
        ("Dried Apricots", "grams", 64.0, 0.0),
        ("Currants", "grams", 85.0, 0.0),
        ("Potato Flakes, Dried", "grams", 21.0, 0.0),
        ("Egg White", "each", 1.0, 0.0)
    ];

    for (name, unit, quantity, cost_per_unit) in sample_inventory {
        conn.execute(
            "INSERT INTO inventory (name, unit, quantity, cost_per_unit)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, unit, quantity, cost_per_unit],
        )?;
    }

    println!("✅ Sample inventory seeded");
    Ok(())
}

// seed recipe data
pub fn seed_recipes(conn: &Connection) -> Result<()> {
    let sample_recipes = vec![
        (
            "Croissant aux Amandes",
            "1. Prepare croissant dough\n2. Make almond cream\n3. Fill, bake, dust with powdered sugar",
            12, "Pastry", "20 minutes", "25 minutes", "4 hours 30 minutes"
        ),
        (
            "Babka",
            "1. Make brioche dough\n2. Prepare chocolate filling\n3. Shape and proof\n4. Bake at 350F",
            8, "Bread", "20 minutes", "25 minutes", "4 hours 30 minutes"
        ),
        (
            "Cinnamon Rolls",
            "1. Roll out dough\n2. Spread cinnamon sugar filling\n3. Slice and bake\n4. Ice while warm",
            10, "Pastry", "20 minutes", "25 minutes", "4 hours 30 minutes"
        ),
    ];

    for (name, instructions, yield_quantity, category, prep_time, bake_time, total_time) in sample_recipes {
        conn.execute(
            "INSERT INTO recipes (name, instructions, yield_quantity, category, prep_time, bake_time, total_time)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?)",
            params![name, instructions, yield_quantity, category, prep_time, bake_time, total_time],
        )?;
    }

    println!("✅ Sample recipes seeded");
    Ok(())
}

// Seed transaction table
pub fn seed_transactions(conn: &Connection) -> Result<()> {
    let transactions = vec![
        ("2025-04-01", "sale", 125.50, "Morning pastry sales"),
        ("2025-04-01", "expense", 42.00, "Purchased 50 lbs of flour"),
        ("2025-04-02", "sale", 98.25, "Coffee + croissant combo special"),
        ("2025-04-02", "expense", 28.75, "Eggs and butter from supplier"),
        ("2025-04-03", "sale", 145.00, "Custom catering order for local office"),
        ("2025-04-03", "expense", 12.99, "Vanilla bean restock"),
        ("2025-04-04", "sale", 162.30, "Saturday morning rush sales"),
        ("2025-04-05", "expense", 80.00, "Marketing design for new packaging"),
        ("2025-04-05", "sale", 73.40, "Farmer's Market pastries"),
        ("2025-04-06", "sale", 84.15, "Sunday brunch box orders"),
    ];

    for (date, tx_type, amount, description) in transactions {
        conn.execute(
            "INSERT INTO transactions (date, transaction_type, amount, description) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![date, tx_type, amount, description],
        )?;
    }

    Ok(())
}

// Recipe_Ingredients seed
pub fn seed_recipe_ingredients(conn: &Connection) -> Result<()> {
    let entries = vec![
        // Croissant aux Amandes (recipe_id = 1)
        (1, 1, 0.75),  // Flour
        (1, 3, 0.50),  // Butter
        (1, 4, 0.25),  // Eggs

        // Babka (recipe_id = 2)
        (2, 1, 1.00),  // Flour
        (2, 2, 0.50),  // Sugar
        (2, 3, 0.50),  // Butter
        (2, 4, 0.25),  // Eggs

        // Cinnamon Rolls (recipe_id = 3)
        (3, 1, 0.50),  // Flour
        (3, 2, 0.25),  // Sugar
        (3, 4, 0.50),  // Eggs
    ];

    for (recipe_id, ingredient_id, qty_required) in entries {
        let result = conn.execute(
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity_required) VALUES (?1, ?2, ?3)",
            params![recipe_id, ingredient_id, qty_required],
        );
    
        match result {
            Ok(_) => println!("✅ Inserted: Recipe {} + Ingredient {} ({})", recipe_id, ingredient_id, qty_required),
            Err(e) => println!("❌ Failed to insert: Recipe {}, Ingredient {} → {}", recipe_id, ingredient_id, e),
        }
    }
    Ok(())
}

// Function to seed tables if empty
pub fn should_seed(conn: &Connection) -> bool {
    let inventory_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM inventory", [], |row| row.get(0))
        .unwrap_or(0);

    let recipe_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM recipes", [], |row| row.get(0))
        .unwrap_or(0);

    let ingredients_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM recipe_ingredients", [], |row| row.get(0))
        .unwrap_or(0);

    inventory_count == 0 || recipe_count == 0 || ingredients_count == 0
}

// Read inventory
pub fn get_all_inventory(conn: &Connection) -> Result<Vec<InventoryItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, unit, quantity, cost_per_unit FROM inventory"
    )?;

    let inventory_iter = stmt.query_map([], |row: &Row| {
        Ok(InventoryItem {
            id: row.get(0)?,
            name: row.get(1)?,
            unit: row.get(2)?,
            quantity: row.get(3)?,
            cost_per_unit: row.get(4)?,
        })
    })?;

    let mut inventory = Vec::new();
    for item in inventory_iter {
        inventory.push(item?);
    }

    Ok(inventory)
}

// Read recipes
pub fn get_recipe_collection(conn: &Connection) -> Result<Vec<RecipeCollection>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, instructions, yield_quantity, category, prep_time, bake_time, total_time, msrp_per_unit FROM recipes"
    )?;

    let recipe_iter = stmt.query_map([], |row: &Row| {
        Ok(RecipeCollection {
            id: row.get(0)?,
            name: row.get(1)?,
            instructions: row.get(2)?,
            yield_quantity: row.get(3)?,
            category: row.get(4)?,
            prep_time: row.get(5)?,
            bake_time: row.get(6)?,
            total_time: row.get(7)?,
            msrp_per_unit: row.get(8)?,


        })
    })?;

    let mut recipes: Vec<RecipeCollection> = Vec::new();
    for recipe in recipe_iter {
        recipes.push(recipe?);
    }

    Ok(recipes)
}

// Function returns all transactions
pub fn read_transactions(conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare("SELECT id, date, transaction_type, amount, description FROM transactions")?;

    let transaction_iter = stmt.query_map([], |row: &Row| {
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            transaction_type: row.get(2)?,
            amount: row.get(3)?,
            description: row.get(4)?,
          })
    })?;

    let mut transactions: Vec<Transaction> = Vec::new();
    for transaction in transaction_iter{
        transactions.push(transaction?);
    }

    Ok(transactions)
}

// Function to filter transactions and return 
pub fn transaction_filter(conn: &Connection, query: &str) ->Result<Vec<Transaction>> {
    
    let mut stmt = conn.prepare(
        "SELECT id, date, transaction_type, amount, description FROM transactions 
        WHERE transaction_type = ?1")?;
    

    let transaction_iter = stmt.query_map([query], |row: &Row| {
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            transaction_type: row.get(2)?,
            amount: row.get(3)?,
            description: row.get(4)?,
            })
    })?;

    let mut transactions: Vec<Transaction> = Vec::new();
    for transaction in transaction_iter{
        transactions.push(transaction?);
    }

    Ok(transactions)
}

// Filter transactions by date
pub fn filter_by_date(conn: &Connection, query: &str) ->Result<Vec<Transaction>> {
    
    let mut stmt = conn.prepare(
        "SELECT id, date, transaction_type, amount, description FROM transactions 
        WHERE date = ?1")?;
    

    let transaction_iter = stmt.query_map([query], |row: &Row| {
        Ok(Transaction {
            id: row.get(0)?,
            date: row.get(1)?,
            transaction_type: row.get(2)?,
            amount: row.get(3)?,
            description: row.get(4)?,
            })
    })?;

    let mut transactions: Vec<Transaction> = Vec::new();
    for transaction in transaction_iter{
        transactions.push(transaction?);
    }

    Ok(transactions)
}

// Function to add an item to inventory
pub fn add_inventory_item(
        conn: &Connection,
        name: &str,
        unit: &str,
        quantity: f32,
        cost_per_unit: f32,
    ) -> Result<()> {
        conn.execute("INSERT INTO inventory (name, unit, quantity, cost_per_unit) VALUES (?1, ?2, ?3, ?4)",
        params![name, unit, quantity, cost_per_unit],
    )?;
    
    Ok(())
}

// Function to update an inventory quantity
pub fn update_inventory_quantity(conn: &Connection, item_id: i32, updated_quantity: f32) -> Result<()> {
    conn.execute("UPDATE inventory SET quantity = ?1 WHERE id = ?2",
    params![updated_quantity, item_id],
    )?;

    Ok(())
}

// Function to update inventory cost_per_unit
pub fn update_inventory_cost(conn: &Connection, item_id: i32, updated_cost: f32) -> Result<()> {
    conn.execute("UPDATE inventory SET cost_per_unit = ?1 WHERE id = ?2",
    params![updated_cost, item_id],
    )?;

    Ok(())

}

// Function to add transaction to database
pub fn add_transaction(
    conn: &Connection,
    date: &str,
    transaction_type: &str,
    amount: f32,
    description: &str,
    ) -> Result<()> {
    conn.execute("INSERT INTO transactions (date, transaction_type, amount, description) VALUES (?1, ?2, ?3, ?4)",
    params![date, transaction_type, amount, description],
    )?;
    
    Ok(())
}

pub fn get_ingredients_for_recipe(conn: &Connection, recipe_id: i32) -> Result<Vec<(String, f32, String)>> {
    let mut stmt = conn.prepare(
        "SELECT i.name, ri.quantity_required, i.unit
         FROM recipe_ingredients ri
         JOIN inventory i ON ri.ingredient_id = i.id
         WHERE ri.recipe_id = ?1"
    )?;

    let rows = stmt.query_map([recipe_id], |row| {
        Ok((
            row.get(0)?, // name
            row.get(1)?, // quantity_required
            row.get(2)?, // unit
        ))
    })?;

    let mut ingredients = Vec::new();
    for row in rows {
        ingredients.push(row?);
    }

    Ok(ingredients)
}

pub fn reset_database(conn: &Connection) -> Result<()> {

    conn.execute("DELETE FROM recipe_ingredients", [])?;
    conn.execute("DELETE FROM transactions", [])?;
    conn.execute("DELETE FROM recipes", [])?;
    conn.execute("DELETE FROM inventory", [])?;
    Ok(())
}

pub fn calculate_recipe_cost(conn: &Connection, recipe_id: i32) -> Result<f32> {
    // you'll write SQL here and sum cost_per_unit * quantity_required
    let mut stmt = conn.prepare(
        "SELECT SUM(i.cost_per_unit * ri.quantity_required)
        FROM recipe_ingredients ri
        JOIN inventory i ON ri.ingredient_id = i.id
        WHERE ri.recipe_id = ?1"
    )?;

    let total_cost: f32 = stmt.query_row([recipe_id], |row| {row.get(0)}).unwrap_or(0.0);


    Ok(total_cost)
}

pub fn deduct_recipe_from_inventory(conn: &Connection, recipe_id: i32) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT ingredient_id, quantity_required
        FROM recipe_ingredients
        WHERE recipe_id = ?1"
    )?;

    let rows = stmt.query_map([recipe_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, f32>(1)?))  // (ingredient_id, quantity_required)
    })?;

    for row in rows {
        let (ingredient_id, qty_required) = row?;
    

        let current_qty: f32 = conn.query_row(
            "SELECT quantity FROM inventory WHERE id = ?1",
            [ingredient_id],
            |row| row.get(0)
        )?;

        let new_qty = current_qty - qty_required;
        conn.execute(
            "UPDATE inventory SET quantity = ?1 WHERE id = ?2",
            params![new_qty, ingredient_id]
        )?;

        println!("🧾 Ingredient {}: {:.2} → {:.2}", ingredient_id, current_qty, new_qty);
    }

    Ok(())
    
}

pub fn write_csv_transaction_report(conn: &Connection) -> io::Result<()> {

    let transactions: Vec<Transaction> = read_transactions(conn).expect("Failed to retrieve transactions.");

    std::fs::create_dir_all("reports")?;
    let file = File::create("reports/transaction-report.csv")?;

    let mut writer = Writer::from_writer(file);

    for tx in transactions {
        writer.serialize(tx).expect("Failed to write row");
    }

    writer.flush()?;
    println!("Transactions printed to transaction-report.csv");
    Ok(())
}

// Update RecipeCollection table with unit MSRP once generated
pub fn update_msrp_for_recipe(conn: &Connection, recipe_id: i32, msrp_per_unit: f32) -> Result<()> {
    conn.execute(
        "UPDATE recipes SET msrp_per_unit = ?1 WHERE id = ?2",
        params![msrp_per_unit, recipe_id],
    )?;
    Ok(())
}

// Check data integrity in bakery.db
pub fn run_integrity_check(conn: &Connection) -> Result<Vec<String>> {
    let mut issues = Vec::new();

    // Check for orphaned recipe ingredients
    let orphan_query = "
        SELECT COUNT(*) FROM recipe_ingredients ri
        LEFT JOIN recipes r ON ri.recipe_id = r.id
        LEFT JOIN inventory i ON ri.ingredient_id = i.id
        WHERE r.id IS NULL OR i.id IS NULL;
    ";
    let orphan_count: i32 = conn.query_row(orphan_query, [], |row| row.get(0))?;
    if orphan_count > 0 {
        issues.push(format!("Found {} orphaned recipe_ingredients entries.", orphan_count));
    }

    // Check for recipes with no ingredients
    let no_ingredient_query = "
        SELECT COUNT(*) FROM recipes r
        LEFT JOIN recipe_ingredients ri ON ri.recipe_id = r.id
        WHERE ri.recipe_id IS NULL;
    ";
    let no_ingredients: i32 = conn.query_row(no_ingredient_query, [], |row| row.get(0))?;
    if no_ingredients > 0 {
        issues.push(format!("Found {} recipes without any ingredients.", no_ingredients));
    }

    Ok(issues)
}

// Vacuum database
pub fn vacuum_database(conn: &Connection) -> Result<()> {
    conn.execute("VACUUM", [])?;
    Ok(())
}

