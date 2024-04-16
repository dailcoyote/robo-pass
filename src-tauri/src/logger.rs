use chrono::Local;
use std::fs;
use std::path::Path;

pub fn setup_logger(logs_folder: &Path) -> Result<(), fern::InitError> {
    if !logs_folder.exists() {
        fs::create_dir(&logs_folder)?;
    }

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "|robo-pass| [{} {} {}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(logs_folder.join("robo-pass.log"))?)
        .apply()?;
    Ok(())
}
