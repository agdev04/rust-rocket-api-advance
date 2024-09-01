
use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", rocket::routes![ 
        rocket_my_advance_practice::routes::options,
        rocket_my_advance_practice::routes::authorization::me,
        rocket_my_advance_practice::routes::authorization::login,
        rocket_my_advance_practice::routes::rustaceans::get_rustaceans,
        rocket_my_advance_practice::routes::rustaceans::get_rustacean,
        rocket_my_advance_practice::routes::rustaceans::create_rustacean,
        rocket_my_advance_practice::routes::rustaceans::update_rustacean,
        rocket_my_advance_practice::routes::rustaceans::delete_rustacean,
        rocket_my_advance_practice::routes::crates::get_crates,
        rocket_my_advance_practice::routes::crates::get_crate,
        rocket_my_advance_practice::routes::crates::create_crate,
        rocket_my_advance_practice::routes::crates::update_crate,
        rocket_my_advance_practice::routes::crates::delete_crate,
    ])
    .attach(rocket_my_advance_practice::routes::Cors)
    .attach(rocket_my_advance_practice::routes::CacheConn::init())
    .attach(rocket_my_advance_practice::routes::DbConn::init())
    .launch()
    .await;
}