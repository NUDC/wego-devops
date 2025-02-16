//! API返回结构体

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Result<T> {
    pub data: Option<T>,
    pub code: i32,
    pub message: Option<String>,
}

impl<T> IntoResponse for Result<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl<T> Result<T>
where
    T: Serialize,
{
    pub fn new(data: Option<T>, code: i32, msg: Option<String>) -> Result<T> {
        Self {
            code,
            message: msg,
            data,
        }
    }

    pub fn ok(data: T) -> Result<T> {
        Result::<T>::new(Some(data), 1, None)
    }

    pub fn err(msg: &str, code: i32) -> Result<T> {
        Result::<T>::new(None, code, Some(msg.to_string()))
    }
}

impl<T> From<anyhow::Error> for Result<T>
where
    T: Serialize,
{
    fn from(value: anyhow::Error) -> Self {
        Result::err(value.to_string().as_str(), -1)
    }
}

impl<T> From<anyhow::Result<T>> for Result<T>
where
    T: Serialize,
{
    fn from(value: anyhow::Result<T>) -> Self {
        match value {
            Ok(data) => Result::ok(data),
            Err(err) => err.into(),
        }
    }
}

// try_trait_v2 实现
use std::{convert::Infallible, ops::FromResidual};

impl<T> FromResidual<anyhow::Result<Infallible>> for Result<T>
where
    T: Serialize,
{
    fn from_residual(residual: anyhow::Result<Infallible>) -> Self {
        match residual {
            Ok(_) => Result::new(None, 1, None),
            Err(err) => err.into(),
        }
    }
}

impl<T> FromResidual<anyhow::Error> for Result<T>
where
    T: Serialize,
{
    fn from_residual(residual: anyhow::Error) -> Self {
        residual.into()
    }
}
