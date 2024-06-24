// Use the `Category` enum from the nested `category` module
pub use category::Category;
// Define a public struct `Product`
pub struct Product {
    name: String,
    id: u64,
    price: f64,
    category: Category,
}

// Define a nested module named `category`
pub mod category;

// Implement methods for the `Product` struct
impl Product {
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {id,name,price,category}
    }
    // Private method to calculate tax
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    // Public method to get the product price including tax
    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}
