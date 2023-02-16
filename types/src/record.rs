use marine_rs_sdk::marine;
use marine_sqlite_connector::{Error, Result, Value};

pub fn get_none_error() -> Error {
  Error {
      code: None,
      message: Some("Value doesn't exist".to_string()),
  }
}

#[marine]
#[derive(Default, PartialEq, Debug)]
pub struct Record {
    pub uuid: i64,
    pub wallet_address: String,
    pub public_key: String,
    pub private_key: String,
    pub err_msg: String,
    pub success: bool,
}

impl Record {
  pub fn from_row(row: &[Value]) -> Result<Option<Record>> {
      let row_item = Record {
          uuid: row[0].as_integer().ok_or(get_none_error())?,
          wallet_address: row[1].as_string().ok_or(get_none_error())?.to_string(),
          public_key: row[2].as_string().ok_or(get_none_error())?.to_string(),
          private_key: row[3].as_string().unwrap_or_default().to_string(),
          err_msg: "".to_string(),
          success: true,
      };

      Ok(Some(row_item))
  }

  pub fn from_res(res: Result<Option<Record>>) -> Record {
      match res {
          Ok(v) => {
            v.unwrap().into()
          },
          Err(e) => {
              let mut res_item: Record = Default::default();
              res_item.err_msg = e.to_string();
              res_item.success = false;
              res_item
          }
      }
  }
}
