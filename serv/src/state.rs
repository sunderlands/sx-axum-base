use anyhow::Context;
use log::error_with_context;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct State {
    db: Option<DatabaseConnection>,
}
impl State {
    pub async fn new() -> anyhow::Result<Self> {
        let instance = State { db: None };
        instance.connet_db().await
    }

    async fn connet_db(mut self) -> anyhow::Result<Self> {
        if let Some(info) = &config::instance().database {
            tracing::info!("开始连接数据库");
            self.db = Some(
                db::connect(&info.url)
                    .await
                    .with_context(error_with_context!("连接数据库时发生异常"))?,
            );
            tracing::info!("连接数据库成功")
        }
        Ok(self)
    }
}
