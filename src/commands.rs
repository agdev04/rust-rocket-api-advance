use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use tera::{Context, Tera};
use crate::auth;
use crate::mail::HTMLMailer;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};
use std::env;
use std::str::FromStr;
use dotenv::dotenv;

fn load_template_engine () -> Tera {
    Tera::new("templates/**/*.html")
    .expect("Cannot load template")
}

async fn load_db_connection() -> AsyncPgConnection {

    dotenv().ok(); // Load environment variables from .env

    let database_url = env::var("DATABASE_URL")
    .expect("Cannot find DATABASE_URL in .env file");

    AsyncPgConnection::establish(&database_url).await
    .expect("Error connecting to the database")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String> ) {
    let mut c = load_db_connection().await;
    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: password_hash.to_string(),
    };

    let role_enums = role_codes.iter().map(|v| RoleCode::from_str(v.as_str()).unwrap()).collect();

    let user = UserRepository::create(&mut c, new_user, role_enums).await.unwrap();
    println!("User created: {:?}", user);
    
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();

    println!("User roles: {:?}", roles);

}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    for user in users {
        println!("User: {:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    UserRepository::delete(&mut c, id).await.unwrap();
}

pub async fn send_digest(email: String, hours_since: i32) {
    let mut c = load_db_connection().await;
    let tera = load_template_engine();
    let crates = CrateRepository::find_since(&mut c, hours_since).await.unwrap();
    if crates.len() > 0 {
        println!("Sending digest to {} with {} crates", email, crates.len());
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);


        dotenv().ok(); // Load environment variables from .env

        let smtp_host = env::var("SMTP_HOST").expect("Cannot find SMTP_HOST in .env file");
        
        let smtp_username = env::var("SMTP_USERNAME").expect("Cannot find SMTP_USERNAME in .env file");
        let smtp_password = env::var("SMTP_PASSWORD").expect("Cannot find SMTP_PASSWORD in .env file");

        let mailer = HTMLMailer { template_engine: tera, smtp_host, smtp_username, smtp_password };
        mailer.send(email, "email/digest.html", context).unwrap();
    }
}