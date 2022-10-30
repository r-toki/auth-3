use crate::{
    lib::{error::Error, module::ModuleExt},
    model::command::user::User,
};

use actix_web::{
    post,
    web::{Json, ServiceConfig},
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize)]
struct Create {
    name: String,
    password: String,
}

#[post("/users")]
async fn create(module: ModuleExt, form: Json<Create>) -> Result<Json<()>, Error> {
    let user = User::create(form.name.clone(), form.password.clone())?;
    module.user_repository.store(user).await?;
    Ok(Json(()))
}
