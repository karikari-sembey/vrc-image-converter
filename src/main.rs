mod config;
mod console_commands;
mod converter;
mod logger;
mod logwatcher;
use config::Config;
use notify::{RecursiveMode, Watcher, recommended_watcher};
use std::{path::Path, sync::mpsc};

#[tokio::main]
async fn main() {
    let config = Config::load();
    config.save();

    logger::init_logger(&config.logger.log_level);
    log::debug!("ログ出力が準備できました！");

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

        tokio::spawn(logwatcher::watch_log(config.clone(), watch_log_receiver));

        console_commands::read_command();
        drop(watcher);
    } else {
        log::error!("ファイル監視システムの起動に失敗しました。");
        return;
    }
}
