pub mod handlers;
pub mod db_connection;
pub mod schema;
pub mod models;

// マクロのインポート(これは今はあまり使わないらしい)
// マクロ: ビルド時にソースコードの一部を置き換える仕組み(よく分からない)
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// actix_webにアクセスし、actix_webモジュール内の要素を全てimportする
extern crate actix_web;
extern crate actix;

use actix_web::{HttpServer, App, web};

fn main () {
  let sys = actix::System::new("mystore");

  HttpServer::new(|| App::new()
    .service(
    // エンドポイントを定義
    web::resource("/products")
      // 非同期処理を登録
      .route(web::get().to_async(handlers::products::index))
      .route(web::post().to_async(handlers::products::create))
  )
    .service(
    web::resource("/products/{id}")
      .route(web::get().to_async(handlers::products::show))
      .route(web::delete().to_async(handlers::products::destroy))
      .route(web::patch().to_async(handlers::products::update))
  ))
      // bindするportを選択
      .bind("0.0.0.0:8000")
      // 値がなかったときにpanicを返す/  Ok なら中の値を返し、Err なら panic を起こす
      .unwrap()
      // 起動
      .start();

  println!("println!(Started http server: http://localhost:8000/products");
  let _ = sys.run();
}