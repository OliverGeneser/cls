use std::{collections::HashMap, fs, path::Path};

use ffprobe::FfProbe;

use crate::state::{get_app, get_mut_app};

pub mod global_input;
pub mod pulseaudio;
pub mod threads;

pub fn ffprobe_info(path: &str) -> Option<FfProbe> {
	let result = ffprobe::ffprobe(path);
	match result {
		Ok(info) => {
			if info.streams.iter().any(|stream| stream.codec_type == Option::Some("audio".to_string())) {
				return Option::Some(info);
			} else {
				return Option::None;
			}
		},
		Err(_err) => {
			// not a media file
			return Option::None;
		}
	}
}

pub fn scan_tab(index: usize) -> Result<(), std::io::Error> {
	let app = get_mut_app();
	if index >= app.config.tabs.len() {
		return Ok(());
	}
	let tab = app.config.tabs[index].clone();
	let mut files = vec![];
	let path = Path::new(tab.as_str());
	if path.is_dir() {
		for entry in fs::read_dir(path)? {
			let file = entry?;
			let longpath = file.path();
			let filename = longpath.file_name().unwrap().to_os_string().into_string().unwrap();
			let filepath = longpath.into_os_string().into_string().unwrap();
			let info = ffprobe_info(filepath.as_str());
			if info.is_some() {
				let duration = info.unwrap().format.get_duration();
				let duration_str: String;
				if duration.is_some() {
					duration_str = humantime::format_duration(duration.unwrap()).to_string();
				} else {
					duration_str = String::new();
				}
				files.push((filename, duration_str));
			}
		}
    files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
		app.files.as_mut().unwrap().insert(tab, files);
	}
	Ok(())
}

pub fn scan_tabs() -> Result<(), std::io::Error> {
	let app = get_mut_app();
	app.files = Option::Some(HashMap::default());
	for ii in 0..app.config.tabs.len() {
		scan_tab(ii)?;
	}
	Ok(())
}

pub fn selected_file_path() -> String {
	let app = get_app();
	if app.files.is_none() {
		return String::new();
	}
	if app.tab_selected >= app.config.tabs.len() {
		return String::new();
	}
	let tab = app.config.tabs[app.tab_selected].clone();
	let files = app.files.as_ref().unwrap().get(&tab);
	if files.is_none() {
		return String::new();
	}
	let unwrapped = files.unwrap();
	if app.file_selected >= unwrapped.len() {
		return String::new();
	}
	return Path::new(&tab).join(&unwrapped[app.file_selected].0).into_os_string().into_string().unwrap();
}