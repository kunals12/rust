use modules::{Customer, Product, Category, Order};
fn main() {
    let customer_1 = Customer::new(1, String::from("Kunal"), String::from("abc@gmail.com"));
    let (name, email) = customer_1.get_customer_data();
    println!("Customer name is {} and email is {}", name, email);

    let prod_1 = Product::new(1, String::from("Rust Book"), 99.99, Category::Books);
    println!("Product price is {}", prod_1.product_price());

    let order_1 = Order::new(1, prod_1, customer_1, 6);
    println!("total bill is {}", order_1.total_bill());
}
