// src/models.rs

#[derive(Debug)]
pub struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub quantity: f32,
    pub cost_per_unit: f32,
}

#[derive(Debug)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub instructions: String,
    pub yield_quantity: i32,
}

#[derive(Debug)]
pub struct RecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity_required: f32,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: i32,
    pub date: String, // We'll parse this later into a chrono::Date
    pub transaction_type: String,
    pub amount: f32,
    pub description: String,
}
