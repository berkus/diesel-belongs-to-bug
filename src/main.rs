extern crate diesel;
extern crate dotenv;
extern crate diesel_belongs_to_bug;

use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel_belongs_to_bug::models::{User, Category};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ));

    use diesel_belongs_to_bug::schema::users::dsl::users;

    let user: User = users.find(1).first(&connection).expect(
        "Failed to load user",
    );
    let cats = Category::belonging_to(&user).load(&connection).expect(
        "Failed to load cats",
    );
}
