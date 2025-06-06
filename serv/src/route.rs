use axum::Router;

pub async fn route() -> anyhow::Result<Router> {
    // /api 路由起点
    Ok(Router::new().nest("/api", Router::new()))
}
