use axum::body::Bytes;
use axum::http::StatusCode;

pub async fn say_hi() -> String {
    String::from("hi")
}

pub async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
