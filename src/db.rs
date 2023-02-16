use marine_sqlite_connector::{Connection, Result};

use types::{Record, get_none_error};

const DB_PATH: &str = "tmp/account_db.sqlite";

pub fn get_connection() -> Connection {
  Connection::open(DB_PATH).unwrap()
}

pub fn create_account_table(conn: &Connection) -> Result<()> {
  conn.execute(
      "
create table if not exists account (
        uuid INTEGER not null primary key AUTOINCREMENT,
        wallet_address varchar(255) not null,
        public_key TEXT not null,
        private_key TEXT not null
    );
",
  )?;

  Ok(())
}

pub fn delete_account_table(conn: &Connection) -> Result<()> {
  conn.execute(
      "
drop table if exists account;
",
  )?;

  Ok(())
}

pub fn add_record(
  conn: &Connection,
  wallet_address: String,
  public_key: String,
  private_key: String,
) -> Result<Option<Record>> {
  conn.execute(format!(
      "insert into account (wallet_address, public_key, private_key) values ('{}', '{}', '{}');",
      wallet_address, public_key, private_key
  ))?;

  println!(
      "insert into account (wallet_address, public_key, private_key) values ('{}', '{}', '{}');",
      wallet_address, public_key, private_key
  );

  let new_row_id = conn
        .prepare("select last_insert_rowid();")?
        .cursor()
        .next()?
        .ok_or(get_none_error())?[0]
        .as_integer()
        .ok_or(get_none_error())?;

    get_record(conn, new_row_id)
}

pub fn get_record(conn: &Connection, item_id: i64) -> Result<Option<Record>> {
  let mut cursor = conn
      .prepare(format!("select * from account where uuid = {};", item_id))?
      .cursor();

  let row = cursor.next()?;

  if row != None {
      let found_record = Record::from_row(row.unwrap()).unwrap();
      Ok(Some(found_record.unwrap()))
  } else {
      Ok(None)
  }

}

pub fn get_record_by_field(
  conn: &Connection,
  field: String,
  value: &String,
) -> Result<Option<Record>> {
  let mut cursor = conn
      .prepare(format!(
          "select * from account where {} = '{}';",
          field, value
      ))?
      .cursor();

  let row = cursor.next()?;
  if row != None {
      let found_record = Record::from_row(row.unwrap()).unwrap();
      Ok(Some(found_record.unwrap()))
  } else {
      Ok(None)
  }
}