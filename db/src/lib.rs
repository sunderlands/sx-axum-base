use anyhow::Context;
use log::error_with_context;
use sea_orm::{Database, DatabaseConnection};

pub async fn connect(url: &str) -> anyhow::Result<DatabaseConnection> {
    Ok(Database::connect(url)
        .await
        .with_context(error_with_context!(
            "连接数据库时发生异常，链接地址:[{}]",
            url
        ))?)
}
