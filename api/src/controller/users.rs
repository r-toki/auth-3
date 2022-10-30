use actix_web::{
    post,
    web::{Json, ServiceConfig},
};
use serde::Deserialize;

use crate::{lib::error::Error, model::command::user::User};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/users")]
async fn create(form: Json<Create>) -> Result<Json<()>, Error> {
    let user = User::create(form.name.clone(), form.password.clone())?;
    Ok(Json(()))
}
