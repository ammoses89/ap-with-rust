use chrono::{Date, Local};

pub trait Allocator {
    fn allocate(&self, order_quantity: i32);
}

pub struct Batch {
    reference: String,
    sku: String,
    quantity: i32,
    eta: Date<Local>,
    is_shipping: bool,
}

pub struct Order {
    order_reference: String,
    order_lines: Vec<OrderLine>,
}

pub struct OrderLine {
    sku: String,
    quantity: i32,
}

impl Batch {
    pub fn new(reference: &str, sku: &str, quantity: i32, is_shipping: bool) -> Self {
        Batch {
            reference: String::from(reference),
            sku: String::from(sku),
            quantity,
            eta: Local::now().date(),
            is_shipping
        }
    }
}

impl Allocator for Batch {
    fn allocate(&mut self, order_quantity: i32) {
        if order_quantity < self.quantity {
            self.quantity -= order_quantity;
        }
    }
}