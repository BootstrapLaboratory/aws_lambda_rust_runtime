//! This module defines the “API types” that your controllers use.

use serde::{Deserialize, Serialize};

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
