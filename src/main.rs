// extern crate server;
// extern crate diesel;

// use self::server::*;
// use self::models::*;
// use self::diesel::prelude::*;

// fn main() {
//     use server::schema::Users::dsl::*;

//     let connection = establish_connection();
//     let results = Users.filter(AccountID.eq(1))
//         .limit(5)
//         .load::<User>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} users", results.len());
   
//     for user in results {
//         println!("{}", user.Email);
//         println!("----------\n");
//         println!("{}", user.Password);
//     }
// }

// script to read data 



extern crate server;
extern crate diesel;
extern crate argon2;
extern crate rand;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use self::server::*;
use self::models::*;
use std::io::{stdin, Read};
use argon2::{Config};
use self::diesel::prelude::*;
use rand::Rng;
use server::schema::Users::dsl::*;

#[get("/login")]
async fn login() -> impl Responder {

         let connection = establish_connection();
    let results = Users.filter(Email.eq("epic"))
        .limit(10)
        .load::<User>(&connection)
        .expect("Error loading user");

    println!("Displaying {} user", results[0].Email);
   
    let password = b"password";
    

   // println!("{}", hash);
    let matches = argon2::verify_encoded(&results[0].Password, password).unwrap();
  
    println!("{}",matches);

    

    
    HttpResponse::Ok().body("Hello world!")




}
    
    







#[get("/register")]
async fn register() -> impl Responder {
   //create random salt
    let salt = rand::thread_rng().gen::<[u8; 12]>();
    //println!("{:?}", salt);
    let config = Config::default();
    
    let email = "epic";
    
    let password = b"password";
    let hash = argon2::hash_encoded(password, &salt, &config).unwrap();
   // println!("{}", hash);
    let matches = argon2::verify_encoded(&hash, password).unwrap();
  
    println!("{}",matches);

    let connection = establish_connection();



    new_user(&connection, email, &hash);
    println!("\nSaved draft {}", email);
    
    HttpResponse::Ok().body("Hello world!")

}

const EOF: &'static str = "ctrl + z";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



// script to insert data