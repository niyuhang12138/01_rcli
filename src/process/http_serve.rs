//! launch serve
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use colored::Colorize;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};

/// share state
#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

/// launch serve
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr.to_string().bright_purple());

    let dir_service = ServeDir::new(path.clone())
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    // axum router
    let app = Router::new()
        .route("/{*path}", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(HttpServeState { path }));

    // listener tcp port 8080
    let listener = TcpListener::bind(addr).await?;
    // run server
    axum::serve(listener, app).await?;

    Ok(())
}

/// file handler for axum
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {p:?}");
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        // TODO: 如果输入的是一个目录, 则将目录下的所有文件输出组织起来为一个HTML文件
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len().to_string().on_purple());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file {e:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
