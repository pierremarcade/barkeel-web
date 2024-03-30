use axum::http::HeaderMap;

pub mod response;

pub fn get_content_type(headers: HeaderMap) -> String {
    let header_value = headers.get("Content-Type");
    let mut content_type = String::new();
    if let Some(header_value) = header_value {
        content_type = header_value.to_str().unwrap().to_owned();
    }
    content_type
}