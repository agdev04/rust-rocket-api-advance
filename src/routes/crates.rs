use crate::models::{ Crate, NewCrate, User };
use crate::repositories::CrateRepository;
use crate::routes::{DbConn, EditorUser, server_error};
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{ json, Json };
use rocket::serde::json::Value;
use rocket_db_pools::Connection;


#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>, _user:User) -> Result<Value, Custom<Value>> {
    CrateRepository::all(&mut db, 100).await
    .map(|a_crates| json!(a_crates))
    .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32, _user:User) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id).await
    .map(|a_crate| json!(a_crate))
    .map_err(|e| server_error(e.into()))
}

#[rocket::post("/crates", data = "<new_crate>", format = "json")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>, _user:EditorUser) -> Result<Value, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner()).await
    .map(|a_crate| json!(a_crate))
    .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", data = "<a_crate>", format = "json")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, a_crate: Json<Crate>, _user:EditorUser) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner()).await
    .map(|a_crate| json!(a_crate))
    .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32, _user:EditorUser) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id).await
    .map(|_| NoContent)
    .map_err(|e| server_error(e.into()))
}