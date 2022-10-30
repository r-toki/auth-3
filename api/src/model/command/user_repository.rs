use crate::lib::error::Error;
use crate::model::command::user::User;

use derive_new::new;
use sqlx::{query, query_as, PgPool};
use std::sync::Arc;

#[derive(new, Debug)]
pub struct UserRepository {
    pub pool: Arc<PgPool>,
}

impl UserRepository {
    pub async fn find(&self, id: String) -> Result<User, Error> {
        let user = query_as!(
            User,
            r#"
select * from users
where id = $1
            "#,
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    pub async fn store(&self, user: User) -> Result<(), Error> {
        query!(
            r#"
insert into users (id, name, password_hash, refresh_token_hash, created_at, updated_at)
values ($1, $2, $3, $4, $5, $6)
on conflict (id)
do update
set name = $2, password_hash = $3, refresh_token_hash = $4, created_at = $5, updated_at = $6
            "#,
            user.id,
            user.name,
            user.password_hash,
            user.refresh_token_hash,
            user.created_at,
            user.updated_at
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
