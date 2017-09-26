```
$ cargo run

[Running cargo run]
   Compiling diesel-belongs-to-bug v0.1.0 (file:///Users/berkus/diesel-belongs-to-bug)
warning: unused attribute
 --> src/models.rs:5:1
  |
5 | #[has_many(categories)]
  | ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_attributes)] on by default

error[E0599]: no function or associated item named `belonging_to` found for type `diesel_belongs_to_bug::models::Category` in the current scope
  --> src/main.rs:25:16
   |
25 |     let cats = Category::belonging_to(&user).load(&connection).expect(
   |                ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

error: Could not compile `diesel-belongs-to-bug`.
```
