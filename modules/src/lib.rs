// Define a module named `product`
mod product {
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
    mod category {
        // Define a public enum `Category`
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }

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
}

// Define a module named `customer`
mod customer {
    // Define a public struct `Customer`
    pub struct Customer {
        pub id: u64,
        pub name: String,
        pub email: String,
    }
}

// Define a module named `order`
mod order {
    // Use the `Customer` struct from the `customer` module and the `Product` struct from the `product` module
    use crate::{customer::Customer, product::Product};

    // Define a struct `Order`
    struct Order {
        pub id: u64,
        pub product: Product,
        pub customer: Customer,
        pub quantity: u32,
    }

    // Implement methods for the `Order` struct
    impl Order {
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
}
