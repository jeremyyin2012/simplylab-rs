use rocket::response::Responder;
use rocket::{Request, response, Response};
use serde_json::{json, Value};
use rocket::http::{ContentType, Status};
use thiserror::Error;
use std::io;
use okapi::openapi3::{MediaType, RefOr, Responses};
use reqwest::header::InvalidHeaderValue;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponderInner;
use schemars::JsonSchema;
use schemars::schema::SchemaObject;
use serde::{Deserialize, Serialize};
use tokio::task;

#[derive(Error, Debug, serde::Serialize, schemars::JsonSchema, Copy, Clone)]
pub enum Code {
    #[error("示例")]
    Example = 10000,
    #[error("未找到记录")]
    RecordNotFound,
    #[error("未找到用户")]
    UserNotFound,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("此功能暂未实现")]
    NotImplemented,
    #[error("未认证")]
    Unauthorized,
    #[error("无权限")]
    Forbidden,
    #[error("参数错误: {0}")]
    ParamsError(String),
    #[error("服务报错: {0}")]
    ServerError(String),
    #[error("数据库连接报错: 无法获取数据库连接: {0}")]
    DatabaseConnectionError(String),
    #[error("数据库Ping报错: {0}")]
    DatabasePingError(String),
    #[error("上游服务报错: {0}")]
    UpstreamError(String),
    #[error("服务反馈: {0}")]
    Feedback(Code),
    // 以下是由 thiserror 提供的自动错误转换
    #[error("EnvVarError: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("UuidError: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("ChronoParseError: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("RedisError: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("SerdeJsonError: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("UrlError: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("SeaQueryError: {0}")]
    SeaQueryError(#[from] sea_query::error::Error),
    #[error("IOError: {0}")]
    IOError(#[from] io::Error),
    #[error("TaskJoinError: {0}")]
    TaskJoinError(#[from] task::JoinError),
    #[error("mongodb::bson::datetime::Error: {0}")]
    BsonDatetimeError(#[from] mongodb::bson::datetime::Error),
    #[error("mongodb::error::Error: {0}")]
    MongodbError(#[from] mongodb::error::Error),
    #[error("mongodb::bson::oid::Error: {0}")]
    MongodbObjectIdError(#[from] mongodb::bson::oid::Error),
    #[error("InvalidHeaderValue: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    // 其他任何错误
    #[error("AnyhowError: {0}")]
    Other(#[from] anyhow::Error),
}

impl Error {
    fn get_http_status(&self) -> Status {
        match self {
            Error::Unauthorized => Status::Unauthorized,
            Error::Forbidden => Status::Forbidden,
            Error::ParamsError(_) => Status::BadRequest,
            Error::ServerError(_) => Status::InternalServerError,
            Error::DatabaseConnectionError(_) => Status::ServiceUnavailable,
            Error::DatabasePingError(_) => Status::ServiceUnavailable,
            Error::UpstreamError(_) => Status::InternalServerError,
            Error::Feedback(_) => Status::Ok,
            _ => Status::InternalServerError,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ErrorMessage {
    message: String,
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let errormsg = self.to_string();
        println!(
            "Error: Request: {} Response: {:#}",
            req, errormsg
        );
        // 发往sentry的如果有Backtrace则会包含Backtrace，返回给用户的则一定不会有Backtrace
        sentry::capture_error(&self);
        let resp = ErrorMessage {
            message: errormsg
                .split(", Backtrace")
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap()
        };
        let err_response = serde_json::to_string(&resp).unwrap();
        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), std::io::Cursor::new(err_response))
            .ok()
    }
}

pub fn response_err(
    _gen: &mut OpenApiGenerator,
    schema: SchemaObject,
    desc: String,
    example: Option<Value>,
) -> okapi::openapi3::Response {
    okapi::openapi3::Response {
        description: desc.to_owned(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType{
                schema: Some(schema),
                example: example,
                ..Default::default()
            }
        },
        ..Default::default()
    }
}

impl OpenApiResponderInner for Error {
    fn responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let schema = gen.json_schema::<ErrorMessage>();
        Ok(Responses {
            responses: okapi::map! {
                Error::Feedback(Code::Example).get_http_status().to_string() + ": Feedback" => RefOr::Object(
                response_err(gen, schema.clone(), Error::Feedback(Code::Example).to_string(),
                    Some(json!({
                            "message": Error::Feedback(Code::Example).to_string(),
                        })))),

                Error::Unauthorized.get_http_status().to_string() => RefOr::Object(
                response_err(gen, schema.clone(), Error::Unauthorized.to_string(),
                    Some(json!({
                            "message": Error::Unauthorized.to_string(),
                        })))),

                Error::Forbidden.get_http_status().to_string() => RefOr::Object(
                response_err(gen, schema.clone(), Error::Forbidden.to_string(),
                    Some(json!({
                            "message": Error::Forbidden.to_string(),
                        })))),

                Error::ParamsError("".to_string()).get_http_status().to_string() + ": ParamsError" => RefOr::Object(
                response_err(gen, schema.clone(), Error::ParamsError("".to_string()).to_string(),
                    Some(json!({
                            "message": Error::ParamsError("".to_string()).to_string(),
                        })))),

                Error::ServerError("".to_string()).get_http_status().to_string() + ": ServerError" => RefOr::Object(
                response_err(gen, schema.clone(), Error::ServerError("".to_string()).to_string(),
                    Some(json!({
                            "message": Error::ServerError("".to_string()).to_string(),
                        })))),

                Error::UpstreamError("".to_string()).get_http_status().to_string() + ": UpstreamError" => RefOr::Object(
                response_err(gen, schema.clone(), Error::UpstreamError("".to_string()).to_string(),
                    Some(json!({
                            "message": Error::UpstreamError("".to_string()).to_string(),
                        })))),

                Error::NotImplemented.get_http_status().to_string() => RefOr::Object(
                response_err(gen, schema.clone(), Error::NotImplemented.to_string(),
                    Some(json!({
                            "message": Error::NotImplemented.to_string(),
                        })))),

            },
            ..Default::default()
        })
    }
}
