 // Use the `Category` enum from the nested `category` module
 use category::Category;

 // Define a public struct `Product`
 pub struct Product {
     pub id: u64,
     pub name: String,
     pub price: f64,
     pub category: Category,
 }

 // Define a nested module named `category`
 mod category;

 // Implement methods for the `Product` struct
 impl Product {
     // Private method to calculate tax
     fn calculate_tax(&self) -> f64 {
         self.price * 0.1
     }

     // Public method to get the product price including tax
     pub fn product_price(&self) -> f64 {
         self.price + self.calculate_tax()
     }
 }