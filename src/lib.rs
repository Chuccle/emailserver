pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use schema::{Users::email, Microbits::id as m_id, Users::id as u_id};
use self::models::*;
use crate::schema::Microbits::dsl::*;
use crate::schema::Users::dsl::*;

pub fn establish_connection() -> diesel::PgConnection {
    
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    diesel::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}





pub fn get_email_address<'a>(conn: &diesel::PgConnection, f_microbitid:&'a str ) ->  String {
    
        let query1_results = Microbits.filter(m_id.eq(f_microbitid))
        .limit(1)
        .load::<Microbit>(conn)
            .expect("Error loading microbit");
            
            if query1_results.is_empty() {
                return "Microbit does not exist".to_string();    
        } 
        
        else if query1_results[0].user_id.is_none() {
            return "Microbit is not assigned".to_string();
        }
    
        let n_user_id = query1_results[0].user_id.unwrap();
    
        let query2_results = Users.filter(u_id.eq(n_user_id))
        .limit(1)
        .load::<User>(conn)
            .expect("Error loading user");
            

            return query2_results[0].email.to_string();
    
          
}


pub fn get_active_time<'a>(conn: &diesel::PgConnection, f_microbitid:&'a str ) ->  String {
    

    
    let results = Microbits.filter(m_id.eq(f_microbitid))
    .limit(1)
    .load::<Microbit>(conn)
        .expect("Error loading microbit");
        
        if results.is_empty() {
           
            return "Microbit does not exist".to_string();    
    } 
    

     return format!("{}::{}::{}::{}", results[0].active_begin_hours, results[0].active_begin_minutes, results[0].active_end_hours, results[0].active_end_minutes);

}

pub fn check_email_exists<'a>(conn: &diesel::PgConnection, f_email: &'a str) -> bool {
    
    let results = Users.filter(email.eq(f_email))
    .limit(1)
    .load::<User>(conn)
        .expect("Error loading user");
        
        if results.is_empty() {
            return true;    
    } 

return false;

    }

pub fn new_user<'a>(conn: &diesel::PgConnection, f_email: &'a str, f_password: &'a str) -> User {
   
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




pub fn check_microbit_exists<'a>(conn: &diesel::PgConnection, f_microbit_id: &'a str) -> &'a str {
    
    let results = Microbits.filter(m_id.eq(f_microbit_id))
    .limit(1)
    .load::<Microbit>(conn)
        .expect("Error loading microbit");
        
        if results.is_empty() {
            return "Microbit does not exist";    
    } 
    
    else if results[0].user_id.is_none() {
        return "Microbit is not assigned";
    }

    return "Microbit is assigned";

    
}



pub fn new_microbit_owner<'a>(conn: &diesel::PgConnection, f_id:&'a i32,  f_microbitid:&'a str ) -> diesel::QueryResult<usize> {
 
    use schema::Microbits;
    
    diesel::update(Microbits::table)
        .filter(m_id.eq(f_microbitid))
        .set(user_id.eq(f_id))
        .execute(conn)

}
