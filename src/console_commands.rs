use std::io::Write;

pub fn read_command() {
    loop {
        print!("> ");
        if std::io::stdout().flush().is_err() {
            log::warn!("標準出力への書き込みが失敗しました。");
        }

        let mut s = String::new();
        if std::io::stdin().read_line(&mut s).is_err() {
            log::warn!("標準入力からデータを読み込めませんでした。");
        }

        match s.trim_end() {
            "help" | "h" => {
                println!("+----------+----------------------------------------+");
                println!("| コマンド | 説明                                   |");
                println!("+**********+****************************************+");
                println!("| help     | このツールのヘルプを表示します。       |");
                println!("+----------+----------------------------------------+");
                println!("| version  | このツールのバージョンを表示します。   |");
                println!("+----------+----------------------------------------+");
                println!("| quit     | このツールを停止します。               |");
                println!("+----------+----------------------------------------+");
            }
            "version" | "v" => {
                println!("VRChat Image Converter v{}", env!("CARGO_PKG_VERSION"));
            }
            "quit" | "q" | "stop" | "exit" | "\x1b" => {
                log::info!("終了します…");
                return;
            }
            "" => {}
            _ => {
                println!("未知のコマンドです。'help'と入力するとヘルプが表示されます。");
            }
        }
    }
}
