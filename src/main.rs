mod config;
mod console_commands;
mod converter;
mod logger;
mod logwatcher;
use config::Config;
use logwatcher::LogWatcher;
use notify::{RecursiveMode, Watcher, recommended_watcher};
use std::{path::Path, sync::mpsc};

#[tokio::main]
async fn main() {
    let config = Config::load();
    let log_watcher = LogWatcher::new();

    config.save();
    logger::init_logger(&config.logger.log_level);

    let (watch_log_sender, watch_log_receiver) = mpsc::channel();

    if let Ok(mut watcher) = recommended_watcher(watch_log_sender) {
        if watcher
            .watch(
                Path::new(&config.input.log_path),
                RecursiveMode::NonRecursive,
            )
            .is_err()
        {
            log::error!("ログファイルの監視に失敗しました。");
            return;
        }

        tokio::spawn(log_watcher.watch_log(config.clone(), watch_log_receiver));

        console_commands::read_command();
        drop(watcher);
    } else {
        log::error!("ファイル監視システムの起動に失敗しました。");
        return;
    }
}
