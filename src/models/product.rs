use crate::schema::products;
use diesel::{RunQueryDsl, QueryDsl};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>
}

impl Product {
  pub fn find(id: &i32) -> Result<Product, diesel::result::Error> {
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();

    products::table.find(id).first(&connection)
  }

  pub fn destroy(id: &i32) -> Result<(), diesel::result::Error> {
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use crate::schema::products::dsl;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();

    // ?:  Result 型の値の後ろにつけて利用され、値が Ok なら中の値を返し、値が Err なら即座に値を return する
    diesel::delete(dsl::products.find(id))
      .execute(&connection)?;
    // 空の文字列を返す
    Ok(())
  }

  pub fn update(id: &i32, new_product: &NewProduct) -> Result<(), diesel::result::Error> {
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use crate::schema::products::dsl;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();

    diesel::update(dsl::products.find(id))
      .set(new_product)
      .execute(&connection)?;
    Ok(())
  }
}

//  AsChangeset: update.setを渡すための型
#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="products"]
pub struct NewProduct {
  pub name: Option<String>,
  pub stock: Option<f64>,
  pub price: Option<i32>
}

impl NewProduct {
  pub fn create(&self) -> Result<Product, diesel::result::Error> {
    use diesel::RunQueryDsl;;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();
    diesel::insert_into(products::table)
      .values(self)
      .get_result(&connection)
  }
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
  pub fn list() -> Self {
    use diesel::RunQueryDsl;
    use diesel::QueryDsl;
    use crate::schema::products::dsl::*;
    use crate::db_connection::establish_connection;

    let connection = establish_connection();

    let result = products.limit(10)
      .load::<Product>(&connection)
      .expect("Error loading products");

    ProductList(result)
  }
}
