use ::entity::prelude::Users;
use ::entity::users::{ActiveModel, Column, Model};
use sea_orm::*;

#[derive(Debug)]
pub struct UserQuery {
    pub db: DbConn,
}

pub struct UserBody {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserQuery {
    pub async fn get_by_id(&self, id: String) -> Result<Option<Model>, DbErr> {
        let body = Users::find_by_id(id).one(&self.db).await?;

        return Ok(body);
    }
    pub async fn get_by_username(&self, username: String) -> Result<Option<Model>, DbErr> {
        let body = Users::find()
            .filter(Column::Username.eq(&username))
            .one(&self.db)
            .await?;

        return Ok(body);
    }

    pub async fn does_exist(&self, body: &UserBody) -> Result<bool, DbErr> {
        let body = Users::find()
            .filter(
                Condition::any()
                    .add(Column::Username.eq(&body.username))
                    .add(Column::Email.eq(&body.email)),
            )
            .one(&self.db)
            .await?;
        if let Some(_data) = body {
            return Ok(true);
        }
        return Ok(false);
    }
    pub async fn register(&self, data: &UserBody) -> Result<Model, DbErr> {
        let mut active_model = ActiveModel {
            ..Default::default()
        };
        active_model.username = ActiveValue::Set(data.username.clone());
        active_model.email = ActiveValue::Set(data.email.clone());
        active_model.password = ActiveValue::Set(data.password.clone());
        let body = active_model.insert(&self.db).await?;

        return Ok(body);
    }
}
