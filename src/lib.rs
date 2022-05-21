pub mod schema;
pub mod models;


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;
use schema::{Users::email, Microbits::id};
use crate::schema::Microbits::dsl::Microbits;
use schema::Microbits::user_id;
use crate::schema::Users::dsl::Users;



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn check_email_exists<'a>(conn: &PgConnection, f_email: &'a str) -> bool {
    
    let results = Users.filter(email.eq(f_email))
    .limit(1)
    .load::<User>(conn)
        .expect("Error loading user");
        
        if results.is_empty() {
            return true;    
    } 

return false;

    }


    use self::models::{User, NewUser};
pub fn new_user<'a>(conn: &PgConnection, f_email: &'a str, f_password: &'a str) -> User {
   
    use schema::Users;
   

    let new_user = NewUser {
        email: f_email,
        password: f_password,
    };


    diesel::insert_into(Users::table) 
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")

}


use self::models::Microbit;
pub fn check_microbit_exists<'a>(conn: &PgConnection, f_microbit_id: &'a str) -> bool {
    
    let results = Microbits.filter(id.eq(f_microbit_id))
    .limit(1)
    .load::<Microbit>(conn)
        .expect("Error loading microbit");
        
        if results.is_empty() {
            return false;    
    } 
    
    else if (results[0].user_id.is_none()) {
        return true;
    }

    return false;

    
}



pub fn new_microbit_owner<'a>(conn: &PgConnection, f_id:&'a i32,  f_microbitid:&'a str ) -> QueryResult<usize> {
 

println!("{}", f_id);
println!("{}", f_microbitid);

    use schema::Microbits;

    diesel::update(Microbits::table)
        .filter(id.eq(f_microbitid))
        .set(user_id.eq(f_id))
        .execute(conn)

}
