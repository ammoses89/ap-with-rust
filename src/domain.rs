use chrono::{Date, Local};
use std::collections::HashSet;
use std::cmp::Ordering;
use std::ops::{DerefMut, Deref};

pub trait Allocator {
    fn allocate(&mut self, order_line: &OrderLine);
    fn deallocate(&mut self, order_line: &OrderLine);
}

pub struct Batch {
    reference: String,
    sku: String,
    pub(crate) quantity: i32,
    eta: Date<Local>,
    is_shipping: bool,
    allocations: HashSet<String>,
}

pub struct Order {
    order_reference: String,
    order_lines: Vec<OrderLine>,
}

pub struct OrderLine {
    order_reference: String,
    sku: String,
    quantity: i32,
}

impl OrderLine {
    pub fn new(order_reference: &str, sku: &str, quantity: i32) -> Self {
        OrderLine {
            order_reference: String::from(order_reference),
            sku: String::from(sku),
            quantity,
        }
    }
}

impl Batch {
    pub fn new(reference: &str, sku: &str, quantity: i32, is_shipping: bool) -> Self {
        let allocations = HashSet::new();
        Batch {
            reference: String::from(reference),
            sku: String::from(sku),
            quantity,
            eta: Local::now().date(),
            is_shipping,
            allocations,
        }
    }
}


impl Allocator for Batch {
    fn allocate(&mut self, order_line: &OrderLine) {
        let order_id = order_line.order_reference.as_str();

        if self.sku != order_line.sku || self.allocations.contains(order_id) {
            return;
        }

        if order_line.quantity <= self.quantity {
            self.quantity -= order_line.quantity;
            self.allocations.insert(order_id.to_string());
        }
    }

    fn deallocate(&mut self, order_line: &OrderLine) {
        let order_id = order_line.order_reference.as_str();

        if self.sku == order_line.sku && self.allocations.contains(order_id) {
            self.quantity += order_line.quantity;
            self.allocations.remove(order_id);
        }


    }
}

impl Ord for Batch {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.is_shipping, self.eta).cmp(&(other.is_shipping, other.eta))
    }
}

impl PartialOrd for Batch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Batch {
    fn eq(&self, other: &Self) -> bool {
        (self.is_shipping, &self.eta) == (other.is_shipping, &other.eta)
    }
}

impl Eq for Batch { }

pub fn allocate(order_line: &OrderLine, mut batches: Vec<Batch>) -> String {
    let batch_reference= String::from("");
    batches
        .sort_by(|b1, b2| b1.cmp(&b2));


    let preferred_batch_option = batches.iter_mut().next();

    if preferred_batch_option.is_some() {
        let preferred_batch = preferred_batch_option.unwrap();
        preferred_batch.allocate(order_line);
    }

    return batch_reference;
}
