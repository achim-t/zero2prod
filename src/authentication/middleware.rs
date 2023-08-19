use actix_web::error::InternalError;
use uuid::Uuid;

use crate::{
    session_state::TypedSession,
    utils::{e500, see_other},
};

pub async fn reject_anonymous_users(session: TypedSession) -> Result<Uuid, actix_web::Error> {
    match session.get_user_id().map_err(e500)? {
        Some(user_id) => Ok(user_id),
        None => {
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in.");
            Err(InternalError::from_response(e, response).into())
        }
    }
}
