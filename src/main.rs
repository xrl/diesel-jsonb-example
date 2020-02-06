#[macro_use] extern crate diesel;

use diesel::prelude::*;

table! {
    jsonb_example {
        id -> Int4,
        stringy -> Varchar,
        some_json -> Jsonb,
    }
}
use jsonb_example::dsl as jsonb_example_dsl;
use diesel::expression::sql_literal::sql;


#[derive(Queryable,Debug)]
struct JsonbExample {
    id: i32,
    stringy: String,
    some_json: serde_json::Value
}

#[derive(Debug,Queryable)]
struct JsonbJustTheId {
    id: i32
}

fn main() {
    println!("Hello, world!");

    let conn = PgConnection::establish("postgres://postgres@localhost:5432/jsonb_query").expect("hello");

//    let res = jsonb_example_dsl::jsonb_example.filter(sql("some_json->'hello' ? 'to all the people'")).load::<JsonbExample>(&conn).unwrap();
//    let res = jsonb_example_dsl::jsonb_example.select((jsonb_example_dsl::id, jsonb_example_dsl::some_json)).load::<(i32, serde_json::Value)>(&conn).unwrap();
    type SmallerSelect = (i32, String);
    let res = jsonb_example_dsl::jsonb_example.select((jsonb_example_dsl::id, sql("some_json->'hello'"))).load::<SmallerSelect>(&conn).unwrap();

    println!("{:#?}", res);
}
