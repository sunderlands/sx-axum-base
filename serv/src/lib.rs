use anyhow::Context;
use axum::serve;
use log::error_with_context;
use tokio::net::TcpListener;

use crate::route::route;

mod route;
mod state;

#[tokio::main]
pub async fn run() -> anyhow::Result<()> {
    let addr = format!(
        "{}:{}",
        config::instance().server.host,
        config::instance().server.port
    );
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(error_with_context!("绑定监听器失败，监听地址:[{}]", addr))?;
    tracing::info!("监听器绑定成功，监听地址:[{}]", addr);

    serve(
        listener,
        route()
            .await
            .with_context(error_with_context!("创建路由失败"))?,
    )
    .await
    .with_context(error_with_context!("服务启动失败"))?;

    Ok(())
}
