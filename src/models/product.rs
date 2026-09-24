#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub sku: String,
    pub price: f64,
    pub quantity: i32,
}