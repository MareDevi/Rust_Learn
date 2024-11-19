use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use axum::Router;
use tower_http::services::ServeDir;
use tracing::info;
use anyhow::Result;

// use axum::{extract::State, http::StatusCode, routing::get, Path};
// use tracing::warn;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {} on {}", state.path.display(), addr);

    // axum router
    let router = Router::new()
        // .route("/*path", get(file_handler))
        .nest_service("/", dir_service)
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    
    Ok(())
}

// async fn file_handler(State(state): State<Arc<HttpServeState>>, Path(path): Path<String>) -> (StatusCode, String) {
//     let p = std::path::Path::new(&state.path).join(path);
//     info!("GET {}", p.display());
//     if !p.exists() {
//         return (StatusCode::NOT_FOUND, "Not Found".to_string());
//     } else {
//         match tokio::fs::read_to_string(p).await {
//             Ok(content) => {
//                 info!("Read {} bytes", content.len());
//                 return (StatusCode::OK, content);
//             }
//             Err(e) => {
//                 warn!("Error: {}", e);
//                 return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
//             }
//         }
//     }
// }