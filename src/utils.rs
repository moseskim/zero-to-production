use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

// 로깅을 위한 에러의 근본 원인은 보존하면서, 불투명한 500을 반환한다.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// 검증 에러의 사용자 표현을 바디로 하는 400을 반환한다.
// 에러의 근본 원인은 로깅 목적을 위해 보존된다.
pub fn e400<T: std::fmt::Debug + std::fmt::Display>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
