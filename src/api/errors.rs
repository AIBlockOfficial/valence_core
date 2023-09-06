use warp::hyper::StatusCode;

/// API Error structure
#[derive(Debug, Clone)]
pub struct ApiError {
    pub code: StatusCode,
    pub message: ApiErrorType,
    pub id: String,
    pub route: String,
}

impl ApiError {
    pub fn new(code: StatusCode, message: ApiErrorType, id: String, route: String) -> Self {
        ApiError {
            code,
            message,
            id,
            route,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl warp::reject::Reject for ApiError {}

/// Constructs an internal server error response for the API
///
/// ### Arguments
///
/// * `error` - Error message
/// * `route` - Route where the error occurred
pub fn construct_result_error(error: &str, route: &str) -> ApiError {
    ApiError::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        ApiErrorType::Generic(error.to_string()),
        "0".to_string(),
        route.to_string(),
    )
}

/// API Error types
#[derive(Debug, Clone)]
pub enum ApiErrorType {
    Generic(String),
    InvalidSignature,
    DBInsertionFailed,
    CacheInsertionFailed,
    CuckooFilterInsertionFailed,
    CuckooFilterLookupFailed,
}

impl std::fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ApiErrorType::Generic(message) => write!(f, "Generic error: {message}"),
            ApiErrorType::InvalidSignature => write!(f, "Invalid signature"),
            ApiErrorType::DBInsertionFailed => write!(f, "DB insertion failed"),
            ApiErrorType::CacheInsertionFailed => write!(f, "Cache insertion failed"),
            ApiErrorType::CuckooFilterInsertionFailed => {
                write!(f, "Cuckoo filter insertion failed")
            }
            ApiErrorType::CuckooFilterLookupFailed => write!(
                f,
                "Cuckoo filter lookup failed, data for address not found on this Beacon"
            ),
        }
    }
}
