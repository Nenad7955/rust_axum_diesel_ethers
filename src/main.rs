mod schema;
use crate::schema::users;

use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::RunQueryDsl;

use dotenv::dotenv;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
struct User {
    id: String,
    username: String,
    pubkey: String,
}



#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let addr = "postgres://db:db@localhost/db";
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(addr);
    let pool = Pool::builder(config).build().unwrap();

    let mut conn = pool.get().await.unwrap();

// use the connection as ordinary diesel-async connection
    let res = users::table
        .select(User::as_select()).load(&mut conn).await.unwrap();

    println!("{:?}", res);

}

/*2.0 validator block creator

tx in mem pool coinbase

evm dynamic memory 0x40

private to public key*/
