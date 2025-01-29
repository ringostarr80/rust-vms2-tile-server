pub mod open_api_controller;
pub mod tile_controller;

use std::sync::PoisonError;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;
use vms2_tile_db_reader::sources::SQLite;

#[derive(Debug)]
pub struct ApiError {
    pub status: Status,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError {
            status: Status::InternalServerError,
            message: format!("Internal Error: {:?}", e),
        }
    }
}

impl From<PoisonError<std::sync::MutexGuard<'_, SQLite>>> for ApiError {
    fn from(e: PoisonError<std::sync::MutexGuard<'_, SQLite>>) -> Self {
        ApiError {
            status: Status::InternalServerError,
            message: format!("Internal Error: {:?}", e),
        }
    }
}

impl From<vms2_tile_db_reader::error::Error> for ApiError {
    fn from(e: vms2_tile_db_reader::error::Error) -> Self {
        ApiError {
            status: Status::InternalServerError,
            message: format!("Internal Error: {:?}", e),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &Request<'_>) -> Result<Response<'static>, Status> {
        let error_message = ErrorMessage {
            code: self.status.code,
            message: self.message,
        };
        Response::build_from(Json(error_message).respond_to(req)?)
            .status(self.status)
            .ok()
    }
}
