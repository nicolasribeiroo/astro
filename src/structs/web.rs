use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ErrorStruct {
    pub code: String,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorStruct,
}

#[derive(Serialize, Clone)]
pub struct SuccessResponse {
    pub success: bool,
    pub data: serde_json::Value,
}
