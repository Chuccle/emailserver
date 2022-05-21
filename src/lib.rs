pub mod schema;
pub mod models;

#[macro_use]

extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use schema::Users::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn check_email_exists<'a>(conn: &PgConnection, email: &'a str) -> bool {
    
    let results = Users.filter(Email.eq(email))
    .limit(1)
    .load::<User>(conn)
        .expect("Error loading user");
        
        if results.is_empty() {
            println!("User already exists");
            return true;    
    } 

return false;


    }





use self::models::{User, NewUser};

pub fn new_user<'a>(conn: &PgConnection, email: &'a str, password: &'a str) -> User {
   
    use schema::Users;
   

    let new_user = NewUser {
        Email: email,
        Password: password,
    };

    diesel::insert_into(Users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}












