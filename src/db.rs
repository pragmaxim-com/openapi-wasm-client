use rocksdb::{
    MultiThreaded, OptimisticTransactionDB, OptimisticTransactionOptions, Options, WriteOptions,
};
use std::sync::{Arc, RwLock};

use crate::models::Data;

pub type Db = Arc<RwLock<OptimisticTransactionDB<MultiThreaded>>>;

pub async fn init_db() -> Db {
    let mut cf_opts = Options::default();
    cf_opts.create_if_missing(true);

    let existing_cfs =
        OptimisticTransactionDB::<MultiThreaded>::list_cf(&cf_opts, "/tmp/trolo").unwrap_or(vec![]);

    let db =
        OptimisticTransactionDB::<MultiThreaded>::open_cf(&cf_opts, "/tmp/trolo", &existing_cfs)
            .unwrap();

    if existing_cfs.is_empty() {
        for cf in vec!["field1", "field2"].into_iter() {
            db.create_cf(cf, &cf_opts).unwrap();
        }
    }

    Arc::new(RwLock::new(db))
}

pub async fn insert_data(db: Db, data: Data) -> Result<(), Box<dyn std::error::Error>> {
    let db = db.write().unwrap();
    let mut write_options = WriteOptions::default();
    write_options.disable_wal(true);

    let txn: rocksdb::Transaction<OptimisticTransactionDB<MultiThreaded>> =
        db.transaction_opt(&write_options, &OptimisticTransactionOptions::default());
    txn.put_cf(
        &db.cf_handle("field1").unwrap(),
        b"field1_key",
        data.field1.as_bytes(),
    )?;
    txn.put_cf(
        &db.cf_handle("field2").unwrap(),
        b"field2_key",
        data.field2.as_bytes(),
    )?;
    txn.commit()?;
    db.flush()?; // repro of broken flushing, no SST file created

    Ok(())
}

pub async fn get_data(db: Db) -> Result<Data, Box<dyn std::error::Error>> {
    let db = db.read().unwrap();
    let field1 = db
        .get_cf(&db.cf_handle("field1").unwrap(), b"field1_key")?
        .unwrap();
    let field2 = db
        .get_cf(&db.cf_handle("field2").unwrap(), b"field2_key")?
        .unwrap();

    Ok(Data {
        field1: String::from_utf8(field1.to_vec())?,
        field2: String::from_utf8(field2.to_vec())?,
    })
}
