use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status_code: u16,
}

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub error: String,
    pub status_code: u16,
}

impl ApiError {
    pub fn new(error: String, status_code: u16) -> Self {
        ApiError { error, status_code }
    }
    
    pub fn bad_request(error: String) -> Self {
        ApiError::new(error, 400)
    }
    
    pub fn internal_server_error(error: String) -> Self {
        ApiError::new(error, 500)
    }
}

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError::new(error.message, error.code as u16)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Status: {}", self.error, self.status_code)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let error_response = ErrorResponse {
            error: self.error.clone(),
            status_code: self.status_code,
        };
        
        match self.status_code {
            400 => actix_web::HttpResponse::BadRequest().json(error_response),
            401 => actix_web::HttpResponse::Unauthorized().json(error_response),
            403 => actix_web::HttpResponse::Forbidden().json(error_response),
            404 => actix_web::HttpResponse::NotFound().json(error_response),
            422 => actix_web::HttpResponse::UnprocessableEntity().json(error_response),
            500 => actix_web::HttpResponse::InternalServerError().json(error_response),
            _ => actix_web::HttpResponse::BadRequest().json(error_response),
        }
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}
