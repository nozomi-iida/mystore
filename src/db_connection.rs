use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection, PoolError};

// リソースをキャッシュすることでデータベースへの接続を向上できる
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
  // .envファイルを読み込む
  dotenv().ok();

  // DATABASE_URLをenvファイルから読み込まれる
  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be est");

  // &: database_urlの値が格納された領域への参照を引数にする
  // PgConnection::establish(&database_url)
  //   .expect(&format!("Error connecting to {}", database_url))
  // 上よりもinit_poolのほうがリソースをキャッシュすることでデータベースへの接続を向上できる
  init_pool(&database_url).expect("Failed to create pool")
}
