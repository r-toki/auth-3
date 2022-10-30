use crate::model::command::user_repository::UserRepository;

use actix_web::web;
use sqlx::PgPool;
use std::sync::Arc;

pub type ModuleExt = web::Data<Arc<Module>>;

#[derive(Debug)]
pub struct Module {
    pub user_repository: Arc<UserRepository>,
}

impl Module {
    pub fn new(pool: PgPool) -> Self {
        let pool = Arc::new(pool);

        let user_repository = Arc::new(UserRepository::new(Arc::from(pool)));

        Self { user_repository }
    }
}
