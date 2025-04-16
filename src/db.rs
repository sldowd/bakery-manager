// src/db.rs
use rusqlite::{Connection, Result, Row, params};
use crate::models::InventoryItem;
use crate::models::RecipeCollection;
use crate::models::Transaction;

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
            yield_quantity INTEGER NOT NULL
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
    )
}

// seed inventory
pub fn seed_inventory(conn: &Connection) -> Result<()> {
    let sample_inventory = vec![
        ("Flour", "lbs", 50.0, 0.45),
        ("Sugar", "lbs", 25.0, 0.55),
        ("Butter", "lbs", 20.0, 3.25),
        ("Eggs", "dozen", 10.0, 2.95),
        ("Vanilla", "oz", 5.0, 1.75),
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
            12,
        ),
        (
            "Babka",
            "1. Make brioche dough\n2. Prepare chocolate filling\n3. Shape and proof\n4. Bake at 350F",
            8,
        ),
        (
            "Cinnamon Rolls",
            "1. Roll out dough\n2. Spread cinnamon sugar filling\n3. Slice and bake\n4. Ice while warm",
            10,
        ),
    ];

    for (name, instructions, yield_quantity) in sample_recipes {
        conn.execute(
            "INSERT INTO recipes (name, instructions, yield_quantity)
             VALUES (?1, ?2, ?3)",
            params![name, instructions, yield_quantity],
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
        "SELECT id, name, instructions, yield_quantity FROM recipes"
    )?;

    let recipe_iter = stmt.query_map([], |row: &Row| {
        Ok(RecipeCollection {
            id: row.get(0)?,
            name: row.get(1)?,
            instructions: row.get(2)?,
            yield_quantity: row.get(3)?,
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


