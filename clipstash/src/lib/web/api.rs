use std::str::FromStr;

use rocket::{
    http::{CookieJar, Status},
    request::{FromRequest, Outcome},
    serde::json::Json,
    Request, Responder, State,
};
use serde::Serialize;

use crate::{
    data::AppDatabase,
    domain::clip::field::Password,
    service::{self, action},
    ServiceError, ShortCode,
};

use super::{HitCounter, PASSWORD_COOKIE};

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[error("invalid API key format")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String),
}

#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn to_base64(&self) -> String {
        base64::encode(self.0.as_slice())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        base64::decode(key)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),

    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    User(Json<String>),

    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<ApiKeyError>),
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Clip(c) => Self::User(Json(format!("clip parsing error: {}", c))),
            ServiceError::NotFound => Self::NotFound(Json("entity not found".to_owned())),
            ServiceError::Data(_) => Self::Server(Json("a server error occurred.".to_owned())),
            ServiceError::PermissionError(msg) => Self::User(Json(msg)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn server_error() -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((
                Status::InternalServerError,
                ApiError::Server(Json("server error".to_owned())),
            ))
        }

        fn key_error(e: ApiKeyError) -> Outcome<ApiKey, ApiError> {
            Outcome::Failure((Status::BadRequest, ApiError::KeyError(Json(e))))
        }

        match req.headers().get_one(API_KEY_HEADER) {
            None => key_error(ApiKeyError::NotFound("API key not found".to_owned())),
            Some(key) => {
                let db = match req.guard::<&State<AppDatabase>>().await {
                    Outcome::Success(db) => db,
                    _ => return server_error(),
                };
                let api_key = match ApiKey::from_str(key) {
                    Ok(api_key) => api_key,
                    Err(e) => return key_error(e),
                };
                match action::api_key_is_valid(api_key.clone(), db.get_pool()).await {
                    Ok(valid) if valid => Outcome::Success(api_key),
                    Ok(valid) if !valid => {
                        key_error(ApiKeyError::NotFound("API key not found".to_owned()))
                    }
                    _ => server_error(),
                }
            }
        }
    }
}

#[rocket::get("/key")]
pub async fn new_api_key(db: &State<AppDatabase>) -> Result<Json<&str>, ApiError> {
    let api_key = action::generate_api_key(db.get_pool()).await?;
    println!("api key: {}", api_key.to_base64());
    Ok(Json("api key generated.  See logs for details."))
}

#[rocket::get("/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    cookies: &CookieJar<'_>,
    hit_counter: &State<HitCounter>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    let req = service::ask::GetClip {
        shortcode: shortcode.clone(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .and_then(|raw_password| Password::new(raw_password.to_string()).ok())
            .unwrap_or_default(),
    };
    let clip = action::get_clip(req, database.get_pool()).await?;
    hit_counter.hit(shortcode, 1);
    Ok(Json(clip))
}

#[rocket::post("/", data = "<req>")]
pub async fn new_clip(
    req: Json<service::ask::NewClip>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    let clip = action::new_clip(req.into_inner(), database.get_pool()).await?;
    Ok(Json(clip))
}

#[rocket::put("/", data = "<req>")]
pub async fn update_clip(
    req: Json<service::ask::UpdateClip>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<crate::Clip>, ApiError> {
    let clip = action::update_clip(req.into_inner(), database.get_pool()).await?;
    Ok(Json(clip))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes!(get_clip, new_clip, update_clip, new_api_key)
}

pub mod catcher {
    use rocket::{catch, catchers, serde::json::Json, Catcher, Request};

    #[catch(default)]
    fn default(req: &Request) -> Json<&'static str> {
        eprintln!("General error: {:?}", req);
        Json("something went wrong...")
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> Json<&'static str> {
        eprintln!("Internal error: {:?}", req);
        Json("internal server error")
    }

    #[catch(404)]
    fn page_not_found() -> Json<&'static str> {
        Json("404")
    }

    #[catch(401)]
    fn request_error() -> Json<&'static str> {
        Json("request error")
    }

    #[catch(400)]
    fn missing_api_key() -> Json<&'static str> {
        Json("API key missing or invalid")
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![
            page_not_found,
            internal_error,
            default,
            request_error,
            missing_api_key
        ]
    }
}
