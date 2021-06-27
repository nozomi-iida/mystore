use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
  // .envファイルを読み込む
  dotenv().ok();

  // DATABASE_URLをenvファイルから読み込まれる
  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be est");

  // &: database_urlの値が格納された領域への参照を引数にする
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
