use crate::models::{NewRustacean, Rustacean, User};
use crate::repositories::RustaceanRepository;
use crate::routes::{DbConn, EditorUser, server_error};
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>, _user:User) -> Result<Value, Custom<Value>> {
    RustaceanRepository::all(&mut db, 100).await
    .map(|rustaceans| json!(rustaceans))
    .map_err(|e| server_error(e.into()))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<DbConn>, id: i32, _user:User) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id).await
    .map(|rustacean| json!(rustacean))
    .map_err(|e| server_error(e.into()))
}

#[rocket::post("/rustaceans", data = "<new_rustacean>", format = "json")]
pub async fn create_rustacean(mut db: Connection<DbConn>, new_rustacean: Json<NewRustacean>, _user:EditorUser) -> Result<Value, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner()).await
    .map(|rustacean| json!(rustacean))
    .map_err(|e| server_error(e.into()))
}

#[rocket::put("/rustaceans/<id>", data = "<rustacean>", format = "json")]
pub async fn update_rustacean(mut db: Connection<DbConn>, id: i32, rustacean: Json<Rustacean>, _user:EditorUser) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner()).await
    .map(|rustacean| json!(rustacean))
    .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<DbConn>, id: i32, _user:EditorUser) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id).await
    .map(|_| NoContent)
    .map_err(|e| server_error(e.into()))
}