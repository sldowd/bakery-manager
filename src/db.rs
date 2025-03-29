// src/db.rs

use rusqlite::{Connection, Result};

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

use rusqlite::params;

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


