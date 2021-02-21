mod domain;

#[cfg(test)]
mod tests {
    use crate::domain;
    use crate::domain::Allocator;

    #[test]
    fn test_allocation_with_sufficient_batch_quantity() {
        // Create a batch with quantity
        let mut batch = domain::Batch::new("ref-id", "small-table", 20, true);
        let order_line = domain::OrderLine::new("order-ref-id", "small-table", 2);

        // Create order with quantity
        batch.allocate(&order_line);

        // Ensure allocation subtracts from batch quantity
        assert_eq!(batch.quantity, 18);
    }

    #[test]
    fn test_allocation_with_insufficient_batch_quantity() {

        // Create a batch with quantity
        let mut batch = domain::Batch::new("ref-id", "blue-cushion", 1, true);
        // Create order with quantity
        let order_line = domain::OrderLine::new("order-ref-id", "blue-cushion", 2);

        // Ensure allocation subtracts from batch quantity
        batch.allocate(&order_line);
        assert_eq!(batch.quantity, 1);
    }

    #[test]
    fn test_allocation_when_order_allocation_called_twice() {
        // Create a batch with quantity
        let mut batch = domain::Batch::new("ref-id", "blue-vase", 10, true);
        // Create order with quantity
        let order_line = domain::OrderLine::new("order-ref-id", "blue-vase", 2);

        batch.allocate(&order_line);
        assert_eq!(batch.quantity, 8);
        batch.allocate(&order_line);
        assert_eq!(batch.quantity, 8);
    }

    #[test]
    fn test_cannot_allocate_if_skus_do_not_match() {
        // Create a batch with quantity
        let mut batch = domain::Batch::new("ref-id", "blue-vase", 10, true);
        // Create order with quantity
        let order_line = domain::OrderLine::new("order-ref-id", "blue-car", 2);

        batch.allocate(&order_line);
        assert_eq!(batch.quantity, 10);
    }

    #[test]
    fn test_can_allocate_if_quantities_are_equal() {
        // Create a batch with quantity
        let mut batch = domain::Batch::new("ref-id", "blue-vase", 10, true);
        // Create order with quantity
        let order_line = domain::OrderLine::new("order-ref-id", "blue-vase", 10);

        batch.allocate(&order_line);
        assert_eq!(batch.quantity, 0);
    }

    #[test]
    fn test_can_deallocate_unallocated_lines() {
        // Create a batch with quantity
        let mut batch = domain::Batch::new("ref-id", "blue-vase", 10, true);
        // Create order with quantity
        let order_line = domain::OrderLine::new("order-ref-id", "blue-vase", 10);

        batch.allocate(&order_line);
        assert_eq!(batch.quantity, 0);

        batch.deallocate(&order_line);
        assert_eq!(batch.quantity, 10);
    }

    #[test]
    fn test_allocates_warehouse_batches_before_shipping_batches() {
        let mut allocator = domain::BatchAllocator::new();
        // Create a batch with quantity
        let shipping_batch = domain::Batch::new("ref-id", "blue-vase", 10, true);
        let warehouse_batch = domain::Batch::new("ref-id", "blue-vase", 10, false);
        // Create order with quantity
        let order_line = domain::OrderLine::new("order-ref-id", "blue-vase", 10);

        allocator.add_batch(shipping_batch);
        allocator.add_batch(warehouse_batch);
        allocator.allocate(&order_line);

        let sorted_batches = allocator.get_batches();

        assert_eq!(sorted_batches[0].quantity, 0);
        assert_eq!(sorted_batches[1].quantity, 10);
        assert_eq!(sorted_batches[1].is_shipping, true);

    }
}
