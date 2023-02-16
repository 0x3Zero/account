#![allow(improper_ctypes)]

mod db;
mod keypair;

use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder};
use types::{FdbResult, Record};
// use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {
  WasmLoggerBuilder::new()
      .with_log_level(log::LevelFilter::Info)
      .build()
      .unwrap();
}

#[marine]
pub fn initialize() -> FdbResult {
    let conn = db::get_connection();
    let res = db::create_account_table(&conn);

    FdbResult::from_res(res)
}

#[marine]
pub fn shutdown() -> FdbResult {
    let conn = db::get_connection();
    let res = db::delete_account_table(&conn);
    FdbResult::from_res(res)
}

#[marine]
pub fn get_account(wallet_address: String) -> Record {
  let conn = db::get_connection();
  let result = db::get_record_by_field(&conn, ("wallet_address").to_string(), &wallet_address);

  let mut res_item: Record = Default::default();


  match result {
    Ok(value) => {
      let r: Record;
      if value.is_none() {
        let newkey = keypair::generate_keypair();
        let res = db::add_record(&conn, wallet_address, newkey.pubkey, newkey.privkey).unwrap_or_default();
        r = res.unwrap();
      } else {
        r = value.unwrap();
      }
      res_item.wallet_address = r.wallet_address.clone();
      res_item.public_key = r.public_key.clone();
      res_item.private_key = r.private_key.clone();
      res_item.err_msg = "".to_string();
      res_item.success = true;
    }
    Err(err) => {
      res_item.err_msg = err.message.unwrap();
      res_item.success = false;
    }
  }

  res_item
}