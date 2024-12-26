use axum::{routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

async fn hello() -> &'static str {
    "Hello world"
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn given_root_path_when_get_then_returns_hello_world() {
        let app = create_router();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "Hello world");
    }
}
