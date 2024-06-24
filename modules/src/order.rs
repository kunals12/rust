 // Use the `Customer` struct from the `customer` module and the `Product` struct from the `product` module
 use crate::{customer::{self, Customer}, product::{self, Product}};

 // Define a struct `Order`
 pub struct Order {
     pub id: u64,
     pub product: Product,
     pub customer: Customer,
     pub quantity: u32,
 }

 // Implement methods for the `Order` struct
 impl Order {
    // Create new Order
    pub fn new(id:u64, product:Product, customer:Customer, quantity:u32) -> Order {
        Order {id, product, customer, quantity}
    }

     // Private method to calculate discount
     fn calculate_discount(&self) -> f64 {
         if self.quantity > 5 {
             0.1
         } else {
             0.0
         }
     }

     // Public method to calculate the total bill including discount
     pub fn total_bill(&self) -> f64 {
         let discount = self.calculate_discount();
         let total_before_discount = self.product.product_price() * self.quantity as f64;
         total_before_discount * (1.0 - discount)
     }
 }