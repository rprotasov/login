#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;
pub mod error;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate crypto;
extern crate rand;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = 
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(
            &format!("Error connection to {}", database_url)
        )
} 
