use axum::{
    http::{header, Response, StatusCode, Uri},
    response::{
        sse::{Event, Sse},
        Html, IntoResponse,
    },
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use futures::stream::{self, Stream};
use rust_embed::RustEmbed;
use std::time::Duration;
use tokio_stream::StreamExt as _;


static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "webui/build"]
struct Assets;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sse", get(sse_handler))
        .fallback(static_handler);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server run on: http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler(
    TypedHeader(_user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = stream::iter(0u32..)
        .map(|i| Event::default().json_data(i))
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response<axum::body::Body> {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response<axum::body::Body> {
    (StatusCode::NOT_FOUND, "404").into_response()
}
