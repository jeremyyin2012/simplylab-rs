use std::ops::Deref;
use rocket::form::Form;
use rocket::serde::json::Json;

use rocket::State;
use rocket_okapi::openapi;
use crate::error::{Code, Error};
use crate::model::{Context, GetAiChatResponseInput, GetAiChatResponseOutput, GetChatStatusTodayOutput, GetUserChatHistoryOutput};

use crate::services::Services;
use crate::error::Error::ParamsError;
use crate::providers::Providers;
use crate::store::Store;


#[openapi(tag = "Hello World")]
#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "favicon.ico")]
#[get("/favicon.ico")]
pub async fn favicon() -> &'static str {
    "favicon.ico"
}


/// # Get AI Chat Response
#[openapi(tag = "Chat")]
#[post("/api/v1/get_ai_chat_response", data="<req>")]
pub async fn get_ai_chat_response(store: &State<Store>, req: Json<GetAiChatResponseInput>) -> Result<Json<GetAiChatResponseOutput>, Error> {
    let req = req.into_inner();
    let pvd = Providers::new(store);
    let user = pvd.user().get_user_by_name(req.user_name.clone()).await?;
    if let Some(user) = user {
        let ctx = Context::new(user);
        let svc = Services::new(ctx, pvd);
        let res = svc.chat().get_ai_chat_response(req).await?;
        Ok(Json(res))
    } else {
        Err(Error::Feedback(Code::UserNotFound))
    }

}

/// # Get User Chat History
#[openapi(tag = "Chat")]
#[get("/api/v1/get_user_chat_history?<user_name>&<last_n>")]
pub async fn get_user_chat_history(store: &State<Store>, user_name: String, last_n: i64) -> Result<Json<GetUserChatHistoryOutput>, Error> {
    let pvd = Providers::new(store);
    let user = pvd.user().get_user_by_name(user_name).await?;
    if let Some(user) = user {
        let ctx = Context::new(user);
        let svc = Services::new(ctx, pvd);
        let res = svc.chat().get_user_chat_history(last_n).await?;
        Ok(Json(res))
    } else {
        Err(Error::Feedback(Code::UserNotFound))
    }
}

/// # Get Chat Status Today
#[openapi(tag = "Chat")]
#[get("/api/v1/get_chat_status_today?<user_name>")]
pub async fn get_chat_status_today(store: &State<Store>, user_name: String) -> Result<Json<GetChatStatusTodayOutput>, Error> {
    let pvd = Providers::new(store);
    let user = pvd.user().get_user_by_name(user_name).await?;
    if let Some(user) = user {
        let ctx = Context::new(user);
        let svc = Services::new(ctx, pvd);
        let res = svc.chat().get_chat_status_today().await?;
        Ok(Json(res))
    } else {
        Err(Error::Feedback(Code::UserNotFound))
    }
}
