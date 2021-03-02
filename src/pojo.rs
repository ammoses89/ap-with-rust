use super::schema::batches;

#[derive(Queryable)]
pub struct Batch {
    id: i32,
    sku: String,
    quantity: i32,
    eta: Date<Local>,
    is_shipping: bool,
    allocations: Option<HashSet<String>>,
}

#[derive(Insertable)]
#[table_name="batches"]
pub struct NewBatch {
    pub(crate) sku: String,
    pub(crate) quantity: i32,
    pub(crate) eta: Date<Local>,
    pub(crate) is_shipping: bool,
}
