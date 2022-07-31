use self::server::*;
use server::schema::Users::dsl::*;
use self::diesel::prelude::*;

extern crate server;
extern crate diesel;
extern crate argon2;
extern crate rand;
extern crate serde;
extern crate dotenv;


#[derive(serde::Serialize)]
struct TimesResponse {
    begin_hours: String,
    begin_minutes: String,
    end_hours: String,
    end_minutes: String,}

#[derive(serde::Deserialize)]
struct RegisterPayload {
    p_email: String,
    p_password: String,
    p_microbit_id: String
}

#[derive(serde::Deserialize)]
struct LoginPayload {
    p_email: String,
    p_password: String,
}

#[derive(serde::Deserialize)]
struct EmailPayload {
    p_microbit_id: String
}



#[actix_web::post("/login")]
async fn login(req: actix_web::web::Json<LoginPayload>) -> impl actix_web::Responder {

        let connection = server::establish_connection();

        let results = Users.filter(email.eq(&req.p_email))
        .limit(1)
        .load::<server::models::User>(&connection)
        .expect("Error loading user");

        if results.is_empty() {
            println!("User does not exist");
            return actix_web::HttpResponse::Ok().body("User does not exist");
        }

    println!("Displaying {} user", results[0].email);
   
    // println!("{}", hash);
    let matches = argon2::verify_encoded(&results[0].password, req.p_password.as_bytes()).unwrap();
  
    println!("{}",matches);

    if matches {
        actix_web::HttpResponse::Ok().json(results[0].id)
    } 
    else {
        actix_web::HttpResponse::Ok().json("Wrong password")
    }

}
    
    


#[actix_web::post("/register")]
async fn register(req: actix_web::web::Json<RegisterPayload>) -> impl actix_web::Responder {
 
 let connection = establish_connection();

 if check_microbit_exists(&connection, &req.p_microbit_id) != "Microbit is not assigned" {
    
    return actix_web::HttpResponse::Ok().body("Microbit invalid");
 }

 
 if !(server::check_email_exists(&connection, &req.p_email)) {

    return actix_web::HttpResponse::Ok().body("Email already exists");

 } 

    //create random salt
    let salt = rand::Rng::gen:: <[u8;12]>(&mut rand::thread_rng());
    //println!("{:?}", salt);
    let config = argon2::Config::default();
    

    let hash = argon2::hash_encoded(req.p_password.as_bytes(), &salt, &config).unwrap();
   // println!("{}", hash);
    let matches = argon2::verify_encoded(&hash, req.p_password.as_bytes()).unwrap();
  
    println!("{}",matches);


   let new_user = new_user(&connection, &req.p_email, &hash);
   

    new_microbit_owner(&connection, &new_user.id, &req.p_microbit_id).unwrap();
    
    println!("\nSaved user {}", new_user.id);

    actix_web::HttpResponse::Ok().body("Hello world!")

}


#[actix_web::post("/email")]
async fn post_email(req: actix_web::web::Json<EmailPayload>) -> impl actix_web::Responder {
   
    let connection = establish_connection();

    let email_extraction = get_email_address(&connection, &req.p_microbit_id);

        if email_extraction == "Microbit does not exist" || email_extraction == "Microbit is not assigned" {
            return actix_web::HttpResponse::Ok().body(email_extraction);
        }

        let google_username = std::env::var("GOOGLE_USERNAME")
        .expect("Username must be set");
    
        let google_password = std::env::var("GOOGLE_PASSWORD")
        .expect("Password must be set");

        let recipient_name=email_extraction.split("@").collect::<Vec<&str>>();
        let sender_name = &google_username.split("@").collect::<Vec<&str>>();
        let recipient_string = format!("{} <{}>", recipient_name[0], email_extraction);
        let sender_string = format!("{} <{}>", sender_name[0], google_username);

     let email_message = lettre::Message::builder()
     .from(sender_string.parse().unwrap())
     .to(recipient_string.parse().unwrap())
     .subject("Microbit Alert")
     .body(String::from("Your microbit has detected a high level of movement."))
     .unwrap();

     let creds = lettre::transport::smtp::authentication::Credentials::new(google_username, google_password);
     let mailer = lettre::SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

match lettre::Transport::send(&mailer, &email_message) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {:?}", e),
}



 actix_web::HttpResponse::Ok().body("Email sent")

}


#[actix_web::post("/updateschedule")]

async fn update_schedule(req: actix_web::web::Json<EmailPayload>) -> impl actix_web::Responder {
   
    let connection = establish_connection();

    let active_time = get_active_time(&connection, &req.p_microbit_id);

    if active_time == "Microbit does not exist" {
            
            return actix_web::HttpResponse::Ok().body(active_time);
    }
   
   let times = active_time.split("::").collect::<Vec<&str>>();


    let response = TimesResponse {
          begin_hours: times[0].to_string(),
          begin_minutes: times[1].to_string(),
          end_hours: times[2].to_string(),
          end_minutes: times[3].to_string(),
     };



 actix_web::HttpResponse::Ok().json(response)

}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let g = std::env::var("DATABASE_URL").expect("DATA_SOURCE_URL must be set");
    
println!("{}", g);

    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(register)
            .service(login)
            .service(post_email)
            .service(update_schedule)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}




