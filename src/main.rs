extern crate server;
extern crate diesel;
extern crate argon2;
extern crate rand;
extern crate serde;


use actix_web::post;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use self::server::*;
use self::models::*;
use argon2::Config;
use self::diesel::prelude::*;
use rand::Rng;
use server::schema::Users::dsl::*;
use serde::Deserialize;




#[derive(Deserialize)]
struct Info {
    p_email: String,
    p_password: String,
    p_microbit_id: String
}




#[post("/login")]
async fn login(req: web::Json<Info>) -> impl Responder {


    println!("{}", req.p_email);
    println!("{}", req.p_password);
         let connection = establish_connection();

         let results = Users.filter(email.eq(&req.p_email))
        .limit(1)
        .load::<User>(&connection)
        .expect("Error loading user");


        if results.is_empty() {
            println!("User does not exist");
            return HttpResponse::Ok().body("User does not exist");
        }

    println!("Displaying {} user", results[0].email);
   

    
   // println!("{}", hash);
    let matches = argon2::verify_encoded(&results[0].password, req.p_password.as_bytes()).unwrap();
  
    println!("{}",matches);

    if matches {
        HttpResponse::Ok().json(results[0].id)
    } 
    else {
        HttpResponse::Ok().json("Wrong password")
    }

}
    
    


#[post("/register")]
async fn register(req: web::Json<Info>) -> impl Responder {
 
 
 //TODO: 
    // Run a select query to see if the desired email already exists
    // If it does, return an error
 // Done

 // Now we need to retrieve the request data from the client
 // Done
 
 let connection = establish_connection();

 if !(check_microbit_exists(&connection, &req.p_microbit_id)) {

    return HttpResponse::Ok().body("Microbit invalid");
 }

 

 if !(check_email_exists(&connection, &req.p_email)) {

    return HttpResponse::Ok().body("Email already exists");

 } 


    //create random salt
    let salt = rand::thread_rng().gen::<[u8; 12]>();
    //println!("{:?}", salt);
    let config = Config::default();
    

    let hash = argon2::hash_encoded(req.p_password.as_bytes(), &salt, &config).unwrap();
   // println!("{}", hash);
    let matches = argon2::verify_encoded(&hash, req.p_password.as_bytes()).unwrap();
  
    println!("{}",matches);


   let newUser = new_user(&connection, &req.p_email, &hash);
   

    new_microbit_owner(&connection, &newUser.id, &req.p_microbit_id);
    println!("\nSaved user {}", newUser.id);


    
    HttpResponse::Ok().body("Hello world!")

}



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