use anyhow::Result;
use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::routing::get;
use axum::{body, Router};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "lib1/pkg"]
struct WasmPackage1;

#[derive(RustEmbed)]
#[folder = "lib2/pkg"]
struct WasmPackage2;

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticResources;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(serve_index_html))
        .route("/js/*path", get(serve_embedded_path::<StaticResources>))
        .route("/lib1/*path", get(serve_embedded_path::<WasmPackage1>))
        .route("/lib2/*path", get(serve_embedded_path::<WasmPackage2>));

    Ok(axum::Server::bind(&"127.0.0.1:4444".parse().unwrap())
        .serve(app.into_make_service())
        .await?)
}

async fn serve_index_html() -> Result<Response, StatusCode> {
    serve_embedded_path::<StaticResources>(Path("index.html".to_string())).await
}

async fn serve_embedded_path<Source: RustEmbed>(
    Path(path): Path<String>,
) -> Result<Response, StatusCode> {
    if let Some(content) = Source::get(&path) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        Ok(Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(body::boxed(body::Full::from(content.data)))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
