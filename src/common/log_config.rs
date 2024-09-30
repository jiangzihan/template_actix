use std::env;

use chrono::Utc;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::EnvFilter;


struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Utc::now().to_rfc3339())
    }
}


pub fn init_tracing() {
    let filter: EnvFilter = env::var("RUST_LOG").unwrap_or("warn".to_string()).parse().expect("错误解析RUST_LOG");
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .with_timer(LocalTimer);

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
    // Use a more compact, abbreviated log format
    .compact()
    .with_env_filter(filter)
    // 写入stdout
    .with_writer(std::io::stdout)
    // .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
    // .with_ansi(false)  // 如果日志是写入文件，应将ansi的颜色输出功能关掉
    // Build the subscriber
    .event_format(format)
    .finish();

    // 设置为全局默认配置
    tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
}


#[cfg(test)]
mod unittest {
    use super::*;
    use dotenvy::dotenv;

    #[test]
    fn test_1()->anyhow::Result<()>{
        dotenv()?;
        init_tracing();

        #[derive(Debug)]
        struct Postion {
            x:u32,
            y:u32
        }

        let err = "error!!!!";
        let pos = Postion{x:10,y:20};

        tracing::trace!(target: "app_events", po1 = ?pos, po2 = ?pos.x, "TRACE日志: {:?}", 1);
        
        let span = tracing::span!(tracing::Level::INFO, "app_events");
        let _guard = span.enter();
        tracing::debug!(po1 = ?pos, po2 = ?pos.y, "DEBUG日志: {:?}", 2);
        tracing::info!(po1 = ?pos, po2 = pos.x, "INFO日志: {:?}", 3);
        tracing::warn!(po1 = ?pos, po2 = pos.y, "WARN日志: {:?}", 4);
        tracing::error!(po1 = ?pos, po2 = ?pos, "ERROR日志: {:?}", 5);
        tracing::event!(name: "aname", tracing::Level::INFO,  "事件方式, ?等于{{:?}}意思, err: {}", err);


        Ok(())
    }
}