// src/db.rs

use rusqlite::{Connection, Result, Row, params};
use crate::models::InventoryItem;
use crate::models::RecipeCollection;

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
            type TEXT NOT NULL,
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

// Function to seed recipes if empty
pub fn should_seed(conn: &Connection) -> bool {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM inventory", [], |row| row.get(0))
        .unwrap_or(0);
    count == 0
}