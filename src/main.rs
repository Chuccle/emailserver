extern crate server;
extern crate diesel;
extern crate argon2;
extern crate rand;
extern crate serde;
extern crate dotenv;

use actix_web::post;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use self::server::*;
use self::models::*;
use argon2::Config;
use self::diesel::prelude::*;
use rand::Rng;
use server::schema::Users::dsl::*;
use serde::{Deserialize, Serialize};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use dotenv::dotenv;
use std::env;

#[derive(Serialize)]
struct TimesResponse {
    begin_hours: String,
    begin_minutes: String,
    end_hours: String,
    end_minutes: String,}

#[derive(Deserialize)]
struct RegisterPayload {
    p_email: String,
    p_password: String,
    p_microbit_id: String
}

#[derive(Deserialize)]
struct LoginPayload {
    p_email: String,
    p_password: String,
}

#[derive(Deserialize)]
struct EmailPayload {
    p_microbit_id: String
}



#[post("/login")]
async fn login(req: web::Json<LoginPayload>) -> impl Responder {


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
async fn register(req: web::Json<RegisterPayload>) -> impl Responder {
 
 
 
 let connection = establish_connection();

 if check_microbit_exists(&connection, &req.p_microbit_id) != "Microbit is not assigned" {
    
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


#[post("/email")]
async fn post_email(req: web::Json<EmailPayload>) -> impl Responder {
   
    let connection = establish_connection();

    let email_extraction = get_email_address(&connection, &req.p_microbit_id);

        if email_extraction == "Microbit does not exist" || email_extraction == "Microbit is not assigned" {
            
            return HttpResponse::Ok().body(email_extraction);
        
        }

        
     
        let google_username = env::var("GOOGLE_USERNAME")
        .expect("Username must be set");
    
        let google_password = env::var("GOOGLE_PASSWORD")
        .expect("Password must be set");

        let recipient_name=email_extraction.split("@").collect::<Vec<&str>>();
         
        let sender_name = &google_username.split("@").collect::<Vec<&str>>();
        
        

        let recipient_string = format!("{} <{}>", recipient_name[0], email_extraction);
        let sender_string = format!("{} <{}>", sender_name[0], google_username);


     let email_message = Message::builder()
     .from(sender_string.parse().unwrap())
     .to(recipient_string.parse().unwrap())
     .subject("Microbit Alert")
     .body(String::from("Your microbit has detected a high level of movement."))
     .unwrap();




     let creds = Credentials::new(google_username, google_password);
     let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();



match mailer.send(&email_message) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {:?}", e),
}

 HttpResponse::Ok().body("Email sent")

}


#[post("/updateschedule")]

async fn update_schedule(req: web::Json<EmailPayload>) -> impl Responder {
   
    let connection = establish_connection();

    let active_time = get_active_time(&connection, &req.p_microbit_id);

    if active_time == "Microbit does not exist" {
            
            return HttpResponse::Ok().body(active_time);
    }
   
   let times = active_time.split("::").collect::<Vec<&str>>();


    let response = TimesResponse {
          begin_hours: times[0].to_string(),
          begin_minutes: times[1].to_string(),
          end_hours: times[2].to_string(),
          end_minutes: times[3].to_string(),
     };



 HttpResponse::Ok().json(response)

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let g = env::var("DATABASE_URL").expect("DATA_SOURCE_URL must be set");
    
println!("{}", g);

    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(login)
            .service(post_email)
            .service(update_schedule)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




