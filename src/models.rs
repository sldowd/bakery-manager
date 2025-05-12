// src/models.rs
use serde::Serialize;


#[derive(Debug)]
pub struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub quantity: f32,
    pub cost_per_unit: f32,
}

#[derive(Debug)]
pub struct RecipeCollection {
    pub id: i32,
    pub name: String,
    pub instructions: String,
    pub yield_quantity: i32,
    pub category: String,
    pub prep_time: Option<String>,
    pub bake_time: Option<String>,
    pub total_time: Option<String>,
    pub msrp_per_unit: Option<f32>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity_required: f32,
}



#[derive(Debug, Serialize)]
pub struct Transaction {
    //"Add additional fields to transaction table-- payee, payment type, second field for category"
    pub id: i32,
    pub date: String,
    pub transaction_type: String,
    pub amount: f32,
    pub description: String,
}
