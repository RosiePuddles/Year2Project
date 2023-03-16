//! # File cleaner script

use std::time::{Duration, SystemTime};

use crate::{logger::Logger, logger_wrap};

/// This function handles file cleaning. Any generated files for download are places in `$CWD/tmp`
/// and made available for up to 1 hour after creation. After that point this function will remove
/// them. It will also attempt to remove files it cannot get proper data from.
pub fn clean(logger: Logger<'_>) -> ! {
	if !std::path::Path::new("tmp").exists() {
		if let Err(e) =  std::fs::create_dir("tmp") {
			logger_wrap!(logger.clean, format!("Unable to create tmp dir - {}", e))
		}
	}
	loop {
		'main: loop {
			let cwd = match std::env::current_dir() {
				Ok(cwd) => cwd,
				Err(e) => {
					logger_wrap!(logger.clean, format!("Error getting cwd - {:?}", e));
					break 'main
				}
			};
			let tmp = match std::fs::read_dir(cwd.join("tmp")) {
				Ok(tmp) => tmp,
				Err(e) => {
					logger_wrap!(logger.clean, format!("Error getting cwd/tmp - {:?}", e));
					break 'main
				}
			};
			for file in tmp {
				if let Ok(file) = file {
					match file.file_type() {
						Ok(ft) if ft.is_file() => {}
						_ => continue,
					}
					let meta = match file.metadata() {
						Ok(meta) => meta,
						Err(e) => {
							logger_wrap!(
								logger.clean,
								format!(
									"Unable to read metadata of file {} - {} - Atempting to delete",
									file.path().to_string_lossy(),
									e
								)
							);
							if let Err(err) = std::fs::remove_file(file.path()) {
								logger_wrap!(
									logger.clean,
									format!("Unable to delete file {} - {}", file.path().to_string_lossy(), err)
								)
							}
							continue
						}
					};
					match meta.created() {
						Ok(created) => {
							if SystemTime::now()
								.duration_since(created)
								.expect("How was the file created in the future")
								.as_secs() > 3600
							{
								logger_wrap!(
									logger.clean,
									format!(
										"File older than 1 hour found. Deleting - {}",
										file.path().to_string_lossy()
									)
								);
								if let Err(err) = std::fs::remove_file(file.path()) {
									logger_wrap!(
										logger.clean,
										format!("Unable to delete file {} - {}", file.path().to_string_lossy(), err)
									)
								}
							}
						}
						Err(e) => {
							logger_wrap!(
								logger.clean,
								format!(
									"Unable to read created time of file {} - {} - Atempting to delete",
									file.path().to_string_lossy(),
									e
								)
							);
							if let Err(err) = std::fs::remove_file(file.path()) {
								logger_wrap!(
									logger.clean,
									format!("Unable to delete file {} - {}", file.path().to_string_lossy(), err)
								)
							}
						}
					}
				}
			}
			break
		}
		std::thread::sleep(Duration::from_secs(300))
	}
}