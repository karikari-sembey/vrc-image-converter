use crate::{config::Config, converter::ImageConverter};
use notify::{
    Error, Event, EventKind,
    event::{DataChange, ModifyKind},
};
use regex::Regex;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
    sync::mpsc::Receiver,
};

pub async fn watch_log(config: Config, watch_log_receiver: Receiver<Result<Event, Error>>) {
    let mut read_lines: usize = 0;

    let warn_or_error_line = Regex::new(r" (Warning|Error)").unwrap();
    let screenshot_line = Regex::new(r"\[VRC Camera\] Took screenshot to: (.+)").unwrap();
    let screenshot_info =
        Regex::new(r"VRChat_(\d{4})-(\d{2})-(\d{2})_(\d{2})-(\d{2})-(\d{2}).(\d{3})_(\d+)x(\d+)")
            .unwrap();

    for event in watch_log_receiver.into_iter().flatten() {
        if event.kind != EventKind::Modify(ModifyKind::Data(DataChange::Any)) {
            continue;
        }
        for path in &event.paths {
            if let Some(file_name) = path.file_name()
                && file_name.to_string_lossy().starts_with("output_log_")
            {
                if read_lines == 0 {
                    read_lines = BufReader::new(fs::File::open(path).unwrap())
                        .lines()
                        .count()
                        - 1;
                }

                if let Ok(file) = fs::File::open(path) {
                    let reader = BufReader::new(file);
                    for line in reader.lines().skip(read_lines).flatten() {
                        read_lines += 1;

                        if line.clone().len() < 1024 {
                            if warn_or_error_line.captures(&line).is_some() {
                                continue;
                            } else if let Some(caps) = screenshot_line.captures(&line)
                                && let Some(info) = screenshot_info.captures(&caps[1])
                            {
                                ImageConverter::convert(
                                    Path::new(&format!(
                                        "{0}/{1}-{2}/VRChat_{1}-{2}-{3}_{4}-{5}-{6}.{7}_{8}x{9}.png",
                                        config.input.picture_path,
                                        &info[1],
                                        &info[2],
                                        &info[3],
                                        &info[4],
                                        &info[5],
                                        &info[6],
                                        &info[7],
                                        &info[8],
                                        &info[9],
                                    )),
                                    Path::new(&format!(
                                        "{0}/{1}",
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
                                            .replace("YYYY", &info[9])
                                    )),
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
