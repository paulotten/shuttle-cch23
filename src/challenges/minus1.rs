use axum::http::StatusCode;

pub async fn http_200() -> StatusCode {
    StatusCode::OK
}

pub async fn http_500() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::challenges::minus1::{http_200, http_500};

    #[tokio::test]
    async fn test_http_200() {
        assert_eq!(http_200().await, StatusCode::OK)
    }

    #[tokio::test]
    async fn test_http_500() {
        assert_eq!(http_500().await, StatusCode::INTERNAL_SERVER_ERROR)
    }
}
