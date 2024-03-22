#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate core;
extern crate dotenv;
#[macro_use]
extern crate rocket;

use std::env;
use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::rapidoc::GeneralConfig;
use rocket_okapi::rapidoc::make_rapidoc;
use rocket_okapi::rapidoc::RapiDocConfig;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::make_swagger_ui;
use rocket_okapi::swagger_ui::SwaggerUIConfig;

use crate::services::Services;
use crate::store::Store;

mod services;
mod conf;
mod model;
mod route;
mod providers;
mod store;
mod error;


pub fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

pub fn get_rapidoc() -> RapiDocConfig {
    RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "/openapi.json")],
            ..Default::default()
        },
        ..Default::default()
    }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    let routes = openapi_get_routes![
        route::index,
        route::favicon,
        route::get_ai_chat_response,
        route::get_user_chat_history,
        route::get_chat_status_today,
    ];
    let store = Store::new().await;
    let sentry_dsn = store.config.sentry_dsn.clone();
    let app_env = store.config.app_env.clone();
    let _guard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(app_env.into()),
            send_default_pii: true,
            ..Default::default()
        },
    ));
    let _rocket = rocket::build()
        .manage(store)
        .mount("/", routes)
        .mount("/docs", make_swagger_ui(&get_docs()))
        .mount("/rapidoc", make_rapidoc(&get_rapidoc()))
        .launch()
        .await?;

    Ok(())
}
