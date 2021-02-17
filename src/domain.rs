struct Batch {
    reference: String,
    sku: String,
    quantity: i32,
}

struct Product {
    sku: String,
}

struct Order {
    order_reference: String,
    order_lines: Vec<OrderLine>,
}

struct OrderLine {
    product: Product,
    quantity: i32,
}

struct Allocator {
    available_quantity: i32,
}