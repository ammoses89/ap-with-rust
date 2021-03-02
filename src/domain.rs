use chrono::{Date, Local};
use std::collections::HashSet;
use std::cmp::Ordering;

pub trait Allocator {
    fn allocate(&mut self, order_line: &OrderLine);
    fn deallocate(&mut self, order_line: &OrderLine);
}

pub struct BatchAllocator {
   batches: Vec<Batch>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Batch {
    reference: String,
    pub(crate) sku: String,
    pub quantity: i32,
    pub(crate) eta: Date<Local>,
    pub is_shipping: bool,
    allocations: HashSet<String>,
}

// pub struct Order {
//     order_reference: String,
//     order_lines: Vec<OrderLine>,
// }

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

impl BatchAllocator {
    pub fn new() -> Self {
        BatchAllocator{
            batches: vec![],
        }
    }

    pub fn add_batch(&mut self, batch: Batch) {
        self.batches.push(batch);
    }

    pub fn sort_batches(&mut self) {
        self.batches
            .sort_by(|b1, b2| b1.cmp(&b2));
    }

    pub fn get_batches(&self) -> &Vec<Batch> {
        return &self.batches;
    }
}

impl Allocator for BatchAllocator {

    fn allocate(&mut self, order_line: &OrderLine) {
        self.sort_batches();
        for (idx, batch) in self.batches.iter_mut().enumerate() {
            if idx == 0 {
                batch.allocate(order_line);
            }
        }
    }

    fn deallocate(&mut self, _order_line: &OrderLine) {

    }
}


