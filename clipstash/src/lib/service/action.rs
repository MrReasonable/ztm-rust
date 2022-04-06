use std::convert::TryInto;

use crate::{
    data::{query, DatabasePool},
    Clip, ServiceError,
};

use super::ask::{UpdateClip, NewClip, GetClip};

pub async fn new_clip(req: NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::new_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

pub async fn get_clip(req: GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();
    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() && clip.password != user_password {
        Err(ServiceError::PermissionError("Invalid password".to_owned()))
    } else {
        Ok(clip)
    }
}