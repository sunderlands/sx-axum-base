use anyhow::Context;
use axum::{Router, serve};
use log::error_with_context;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn run() -> anyhow::Result<()> {
    let addr = format!("localhost:3000");
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(error_with_context!("绑定监听器失败，监听地址:[{}]", addr))?;
    tracing::info!("监听器绑定成功，监听地址:[{}]", addr);

    serve(listener, Router::new())
        .await
        .with_context(error_with_context!("服务启动失败"))?;

    Ok(())
}
