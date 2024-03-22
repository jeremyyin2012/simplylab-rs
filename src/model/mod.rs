use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::ops::Deref;
use std::str::FromStr;

use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use schemars::_private::NoSerialize;
use serde::{Deserializer, Serializer};
use serde_json::Value;
use sqlx::types::Uuid;

pub use req::*;
pub use res::*;
pub use table::*;
pub use entity::*;

use crate::error::Error;
use crate::error::Error::ParamsError;

mod deser;
mod entity;
mod req;
mod res;
mod table;
