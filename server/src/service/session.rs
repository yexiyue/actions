pub struct SessionService;
use anyhow::Result;
use entity::session::{self, ActiveModel, Entity as Session, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

impl SessionService {
    pub async fn create_session(db: &DbConn, model: Model) -> Result<Model> {
        let active_model = ActiveModel {
            user_id: Set(model.user_id),
            access_token: Set(model.access_token.clone()),
            refresh_token: Set(model.refresh_token.clone()),
            expires_at: Set(model.expires_at),
            ..Default::default()
        };
        let res = Session::insert(active_model).exec(db).await?;
        Ok(Model {
            id: res.last_insert_id,
            ..model
        })
    }

    pub async fn find_session_by_user_id(db: &DbConn, user_id: i32) -> Result<Option<Model>> {
        Ok(Session::find()
            .filter(session::Column::UserId.eq(user_id))
            .one(db)
            .await?)
    }

    pub async fn update_session(db: &DbConn, model: Model) -> Result<Model> {
        if let Some(session) = Self::find_session_by_user_id(db, model.user_id).await? {
            let mut active_model: ActiveModel = session.into();
            active_model.access_token = Set(model.access_token);
            active_model.refresh_token = Set(model.refresh_token);
            active_model.expires_at = Set(model.expires_at);
            return Ok(active_model.update(db).await?);
        }
        Err(anyhow::anyhow!("Session not found"))
    }

    pub async fn create_or_update_session(db: &DbConn, model: Model) -> Result<Model> {
        match Self::update_session(db, model.clone()).await {
            Ok(updated_session) => Ok(updated_session),
            Err(_) => Self::create_session(db, model).await,
        }
    }
}
