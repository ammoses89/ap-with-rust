use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use schema::batch;
use crate::domain::Batch;
use crate::pojo;

pub(crate) struct EcommerceRepository {
}

impl EcommerceRepository {

    pub fn new(conn: &PgConnection) -> Self {
       EcommerceRepository {
       }
    }

    pub fn addBatch(&self, conn: &PgConnection, batch: &Batch) {
        let new_batch = pojo::NewBatch{
            sku: batch.sku.clone(),
            quantity: batch.quantity,
            eta: batch.eta,
            is_shipping: batch.is_shipping,
        };

        diesel::insert_into(batch::table)
            .values(&new_batch)
            .execute(conn)
            .expect("Failed to insert batch");

    }

    pub fn listBatches(&self) -> Vec<Batch>{
        return vec![];
    }
}