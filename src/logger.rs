use serde::{Deserialize, Serialize};

pub fn init_logger(level: &LogLevel) {
    let default = fern::Dispatch::new();
    let console_config = fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        .format(|out, message, record| {
            out.finish(format_args! {
                "{} [{}]: {}:{} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or_default(),
                record.line().unwrap_or_default(),
                message
            })
        })
        .level(level.clone().into())
        .chain(std::io::stdout());

    if default.chain(console_config).apply().is_err() {
        println!("ログの設定に失敗しました。");
    }
    log::debug!("ログ出力が準備できました！");
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum LogLevel {
    Off,
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(val: LogLevel) -> Self {
        use log::LevelFilter;
        match val {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Trace => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
        }
    }
}
