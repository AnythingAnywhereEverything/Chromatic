use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    body::Body,
    extract::{DefaultBodyLimit, Query, Request},
    http::{HeaderMap, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use axum_client_ip::ClientIpSource;
use serde_json::json;
use tokio::{
    net::TcpListener,
    signal::{
        self,
        unix::{self, SignalKind},
    },
};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    api::{
        error::APIError, routes::{
            auth_routes, dev_routes, message_routes, post_routes, protected_routes, tag_routes, user_routes,
        },
    }, application::state::SharedState,
};

pub async fn create_router(state: SharedState) -> Router {
    // Build a CORS layer.
    // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
    // for more details
    let cors_layer = CorsLayer::new().allow_origin(Any);

    // Build the router.
    let router = Router::new()
        .route("/head", get(head_request_handler))
        .route("/any", any(any_request_handler))
        .route("/{version}/health", get(health_handler))
        .route("/{version}/version", get(version_handler))
        // Nesting authentication routes.
        .nest("/{version}/auth", auth_routes::routes())
        // Nesting user routes.
        .nest("/{version}/users", user_routes::routes())
        .nest("/{version}/posts", post_routes::routes())
        .nest("/{version}/messages", message_routes::routes())
        .nest("/{version}/dev", dev_routes::routes())
        .nest("/{version}/protected", protected_routes::routes())
        .nest("/{version}/tags", tag_routes::routes())
        // Add a fallback service for handling routes to unknown paths.
        .fallback(error_404_handler)
        .with_state(Arc::clone(&state))
        .layer(cors_layer)
        .layer(middleware::from_fn(logging_middleware))
        .layer(ClientIpSource::XRealIp.into_extension())
        .layer(DefaultBodyLimit::disable()); // Let NGINX and route handle the body size limit, as we have a reverse proxy in front of this service.

    router
}

pub async fn start(state: SharedState) {
    // let cors_layer = CorsLayer::new()
    //      .allow_origin(cors_header_value)
    //      .allow_methods([
    //          Method::HEAD,s
    //          Method::GET,
    //          Method::POST,
    //          Method::PATCH,
    //          Method::DELETE,
    //      ])
    //      .allow_credentials(true)
    //      .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Build the router.
    let router = create_router(state.clone()).await;

    // Build the listener.
    let addr = state.config.service_socket_addr();
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);

    // Start the API service.
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("server shutdown successfully.");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("received termination signal, shutting down...");
}

#[tracing::instrument(level = tracing::Level::TRACE, name = "axum", skip_all, fields(method=request.method().to_string(), uri=request.uri().to_string()))]
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    tracing::trace!(
        "received a {} request to {}",
        request.method(),
        request.uri()
    );
    next.run(request).await
}

// Health request handler.
pub async fn health_handler() -> Result<impl IntoResponse, APIError> {
    Ok(Json(json!({"status": "healthy"})))
}

// Version request handler.
pub async fn version_handler() -> Result<impl IntoResponse, APIError> {
    let result = json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
    });
    Ok(Json(result))
}

// A sample head request handler.
// Using HEAD requests makes sense if processing (computing) the response body is costly.
pub async fn head_request_handler(method: Method) -> Response {
    if method == Method::HEAD {
        tracing::debug!("HEAD method found");
        return [("x-some-header", "header from HEAD")].into_response();
    }
    ([("x-some-header", "header from GET")], "body from GET").into_response()
}

// A sample any request handler.
pub async fn any_request_handler(
    method: Method,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    request: Request,
) -> impl IntoResponse {
    if tracing::enabled!(tracing::Level::DEBUG) {
        tracing::debug!("method: {:?}", method);
        tracing::debug!("headers: {:?}", headers);
        tracing::debug!("params: {:?}", params);
        tracing::debug!("request: {:?}", request);
    }
    StatusCode::OK
}

// 404 handler.
pub async fn error_404_handler(request: Request) -> impl IntoResponse {
    tracing::error!("route not found: {:?}", request);
    StatusCode::NOT_FOUND
}
