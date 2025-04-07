// src/db.rs

use rusqlite::Statement;
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
    conn.execute("INSERT INTO transactions (date, type, amount, description) VALUES (?1, ?2, ?3, ?4)",
    params![date, transaction_type, amount, description],
    )?;
    
    Ok(())
}

// Function to seed recipes if empty
pub fn should_seed(conn: &Connection) -> bool {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM inventory", [], |row| row.get(0))
        .unwrap_or(0);
    count == 0
}