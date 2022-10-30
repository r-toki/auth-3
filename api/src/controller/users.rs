use actix_web::{
    post,
    web::{Json, ServiceConfig},
};
use serde::Deserialize;
use validator::Validate;

use crate::lib::error::Error;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(create);
}

#[derive(Debug, Deserialize, Validate)]
struct Create {
    #[validate(length(min = 3, max = 15, message = "must be 3-15 characters long"))]
    name: String,
    #[validate(length(min = 8, max = 30, message = "must be 8-30 characters long"))]
    password: String,
}

#[post("/users")]
async fn create(form: Json<Create>) -> Result<Json<()>, Error> {
    form.validate()?;
    Ok(Json(()))
}
