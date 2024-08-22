use rocksdb::{
    MultiThreaded, OptimisticTransactionDB, OptimisticTransactionOptions, Options, WriteOptions,
};
use std::sync::{Arc, RwLock};

use model::{Address, Block};

pub type Db = Arc<RwLock<OptimisticTransactionDB<MultiThreaded>>>;

fn box_to_array(boxed: Box<[u8]>) -> [u8; 8] {
    // SAFETY: The caller must guarantee that `boxed` has exactly 8 elements.
    let ptr = Box::into_raw(boxed) as *mut [u8; 8];
    unsafe { *Box::from_raw(ptr) }
}

pub async fn init_db() -> Db {
    let mut cf_opts = Options::default();
    cf_opts.create_if_missing(true);

    let existing_cfs =
        OptimisticTransactionDB::<MultiThreaded>::list_cf(&cf_opts, "/tmp/trolo").unwrap_or(vec![]);

    let db =
        OptimisticTransactionDB::<MultiThreaded>::open_cf(&cf_opts, "/tmp/trolo", &existing_cfs)
            .unwrap();

    if existing_cfs.is_empty() {
        for cf in vec!["addresses", "blocks"].into_iter() {
            db.create_cf(cf, &cf_opts).unwrap();
        }
    }

    Arc::new(RwLock::new(db))
}

pub async fn insert_address(db: Db, address: Address) -> Result<(), Box<dyn std::error::Error>> {
    let db = db.write().unwrap();
    let mut write_options = WriteOptions::default();
    write_options.disable_wal(true);

    let txn: rocksdb::Transaction<OptimisticTransactionDB<MultiThreaded>> =
        db.transaction_opt(&write_options, &OptimisticTransactionOptions::default());
    txn.put_cf(
        &db.cf_handle("addresses").unwrap(),
        address.address.as_bytes(),
        u64::to_be_bytes(address.balance),
    )?;
    txn.commit()?;
    db.flush()?; // repro of broken flushing, no SST file created

    Ok(())
}

pub async fn insert_block(db: Db, block: Block) -> Result<(), Box<dyn std::error::Error>> {
    let db = db.write().unwrap();
    let mut write_options = WriteOptions::default();
    write_options.disable_wal(true);

    let txn: rocksdb::Transaction<OptimisticTransactionDB<MultiThreaded>> =
        db.transaction_opt(&write_options, &OptimisticTransactionOptions::default());
    txn.put_cf(
        &db.cf_handle("blocks").unwrap(),
        block.block_id.as_bytes(),
        u64::to_be_bytes(block.height),
    )?;
    txn.commit()?;
    db.flush()?; // repro of broken flushing, no SST file created
    Ok(())
}

pub async fn get_addresses(db: Db) -> Vec<Address> {
    let db = db.read().unwrap();
    let cf_handle = db.cf_handle("addresses").unwrap();
    db.iterator_cf(&cf_handle, rocksdb::IteratorMode::Start)
        .map(|result| match result {
            Ok((address, balance)) => Address {
                address: String::from_utf8(address.to_vec()).unwrap(),
                balance: u64::from_be_bytes(box_to_array(balance)),
            },
            Err(err) => panic!("Error {}", err.to_string()),
        })
        .collect()
}

pub async fn get_blocks(db: Db) -> Vec<Block> {
    let db = db.read().unwrap();
    let cf_handle = db.cf_handle("blocks").unwrap();
    db.iterator_cf(&cf_handle, rocksdb::IteratorMode::Start)
        .map(|result| match result {
            Ok((block_id, height)) => Block {
                block_id: String::from_utf8(block_id.to_vec()).unwrap(),
                height: u64::from_be_bytes(box_to_array(height)),
            },
            Err(err) => panic!("Error {}", err.to_string()),
        })
        .collect()
}
