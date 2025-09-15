use crate::{config::Config, converter::ImageConverter};
use notify::{
    Error, Event, EventKind,
    event::{DataChange, ModifyKind},
};
use regex::Regex;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::mpsc::Receiver,
};

pub struct LogWatcher {
    screenshot_line: Regex,
    screenshot_info: Regex,
}

impl LogWatcher {
    pub fn new() -> Self {
        Self {
            screenshot_line: Regex::new(r"\[VRC Camera\] Took screenshot to: .+(VRChat_.+)")
                .unwrap(),
            screenshot_info: Regex::new(
                r"VRChat_(\d{4})-(\d{2})-(\d{2})_(\d{2})-(\d{2})-(\d{2}).(\d{3})_(\d+)x(\d+)",
            )
            .unwrap(),
        }
    }

    pub async fn watch_log(
        self,
        config: Config,
        watch_log_receiver: Receiver<Result<Event, Error>>,
    ) {
        let mut read_lines: usize = 0;

        for event in watch_log_receiver.into_iter().flatten() {
            if event.kind != EventKind::Modify(ModifyKind::Data(DataChange::Any)) {
                continue;
            }
            for path in &event.paths {
                if let Some(file_name) = path.file_name()
                    && file_name.to_string_lossy().starts_with("output_log_")
                    && let Ok(file) = fs::File::open(path)
                {
                    let reader = BufReader::new(file);
                    for line in reader.lines().skip(read_lines).flatten() {
                        read_lines += 1;

                        if line.clone().len() < 1024 {
                            if line.contains(" Warning    -  ") || line.contains(" Error      -  ")
                            {
                                continue;
                            } else if let Some(caps) = self.screenshot_line.captures(&line)
                                && let Some(info) = self.screenshot_info.captures(&caps[1])
                            {
                                let src_path = PathBuf::from(format!(
                                    "{0}/{1}-{2}/{3}",
                                    config.input.picture_path, &info[1], &info[2], &caps[1]
                                ));
                                let dst_path = PathBuf::from(format!(
                                    "{}/{}.{}",
                                    config.output.save_path,
                                    config
                                        .output
                                        .name
                                        .replace("yyyy", &info[1])
                                        .replace("MM", &info[2])
                                        .replace("dd", &info[3])
                                        .replace("hh", &info[4])
                                        .replace("mm", &info[5])
                                        .replace("ss", &info[6])
                                        .replace("fff", &info[7])
                                        .replace("XXXX", &info[8])
                                        .replace("YYYY", &info[9]),
                                    config.output.codec.file_extension()
                                ));
                                if !fs::exists(&dst_path).unwrap() && fs::exists(&src_path).unwrap()
                                {
                                    ImageConverter::convert(
                                        &src_path,
                                        &dst_path,
                                        &config.output.codec,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
