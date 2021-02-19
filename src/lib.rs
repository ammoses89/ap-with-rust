mod domain;

#[cfg(test)]
mod tests {
    use crate::domain;

    #[test]
    fn test_allocation_with_sufficient_batch_quantity() {
        // Create a batch with quantity
        let batch = domain::Batch::new("ref-id", "sku-hi", 10, true);

        // Create order with quantity
        // Ensure allocation subtracts from batch quantity
    }

    #[test]
    fn test_allocation_with_insufficient_batch_quantity() {
        // Create a batch with quantity
        // Create order with quantity
        // Ensure allocation subtracts from batch quantity
    }

    #[test]
    fn test_allocation_when_order_allocation_called_twice() {
        // Create a batch with quantity
        // Create order with quantity
        // Ensure allocation subtracts from batch quantity
    }
}