// use std::option::Option;
use schema::{categories, users};

#[derive(Queryable, Serialize, Identifiable)]
#[has_many(categories)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[table_name = "categories"]
#[belongs_to(User, foreign_key = "username")]
pub struct Category {
    pub id: i32,
    pub username: String,
}
