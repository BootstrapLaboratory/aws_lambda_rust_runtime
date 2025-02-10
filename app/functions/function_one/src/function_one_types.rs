//! This module defines the “API types” that your controllers use.

use serde::{Deserialize, Serialize};

/// Your application’s Request – you can add more fields as needed (e.g. headers, body).
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// For simplicity we extract just the query parameters.
    pub name: String,
}

/// Your successful Response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

/// Your Error Response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
