use actix_web::HttpResponse;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type AsyncVoidResult = AsyncResult<()>;
pub type APIResult = AsyncResult<HttpResponse>;
