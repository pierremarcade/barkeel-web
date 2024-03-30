use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::http::header::CONTENT_TYPE;

#[derive(Debug)]
pub struct Response<'a> {
    pub datas: String,
    pub status_code: StatusCode,
    pub content_type: &'a str,
}

impl IntoResponse for Response<'_> {
    fn into_response(self) -> axum::response::Response {
        let status_code: StatusCode = self.status_code;
        (
            status_code,
            [
                (CONTENT_TYPE, self.content_type)
            ],
            self.datas
        ).into_response()
    }
}