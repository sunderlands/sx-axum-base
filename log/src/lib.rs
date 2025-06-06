use tracing_appender::{
    non_blocking::{NonBlocking, WorkerGuard},
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// # 将tracing融入anyhow流程
/// 专门用于传入 `anyhow` 的 `with_context` ，且同时进行 `tracing::error!` 记录  
///
/// **注意！若跨项目使用，需要在使用项目中Cargo.tomml引入tracing**
#[macro_export]
macro_rules! error_with_context {
    ($($arg:tt)*) => {{
        let s = format!($($arg)*);
        move || ->String {
            tracing::error!("{}", &s);
            s
        }
    }};
}

pub fn init() -> Vec<WorkerGuard> {
    // 日志输出级别
    let filer = EnvFilter::new(&config::instance().log.level);
    // 时间格式化
    let timer = OffsetTime::new(toolkit::time::OFFSET_BEIJING, toolkit::time::FORMAT_UTIL);

    // 文件输出
    let (file_writer, file_gaurd) = file_writer();
    let file_layer = fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_timer(timer.clone())
        .json();

    let mut rs_vec = vec![file_gaurd];

    // 是否处于调试模式
    match config::instance().log.debug {
        true => {
            // 屏幕标准输出
            let (std_writer, std_guard) = std_writer();
            let std_layer = fmt::layer()
                .with_writer(std_writer)
                .with_timer(timer)
                .pretty();

            rs_vec.push(std_guard);
            tracing_subscriber::registry()
                .with(filer)
                .with(file_layer)
                .with(std_layer)
                .init();
        }
        false => tracing_subscriber::registry()
            .with(filer)
            .with(file_layer)
            .init(),
    }
    tracing::info!("日志模块启动成功");
    rs_vec
}

type NonBlockingWriter = (NonBlocking, WorkerGuard);

fn file_writer() -> NonBlockingWriter {
    tracing_appender::non_blocking(RollingFileAppender::new(
        Rotation::DAILY,
        "./logs",
        "serv.log",
    ))
}

fn std_writer() -> NonBlockingWriter {
    tracing_appender::non_blocking(std::io::stdout())
}
