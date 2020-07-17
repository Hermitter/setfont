use super::Setting;
use crate::{Result, Shared};
use regex::Regex;
use std::io::{Error, ErrorKind, Write};
use std::process::{Command, Stdio};
use std::{env, fs};

// Schema for Tilix dconf settings: https://github.com/gnunn1/tilix/blob/4178cf16f4b15f06b679fa05c0fa6fc8afd40999/source/gx/tilix/preferences.d#L315
/// Profile ID Tilix creates on first launch
const DEFAULT_PROFILE_ID: &str = "2b7c4080-0ddd-46c5-8f23-563fd3ba789d";
/// Tilix's default setting for font family
const DEFAULT_FONT_FAMILY: &str = "Monospace Regular";
/// Tilix's default setting for font size
const DEFAULT_FONT_SIZE: &str = "12";

pub fn apply(setting: &Setting, _shared: &Shared) -> Result {
    // Ensure Tilix & dconf are installed.
    if !is_program_in_path("tilix") && !is_program_in_path("dconf") {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Could not find Tilix installation",
        )));
    }

    let font = match setting.font() {
        Some(f) => f.as_str(),
        None => DEFAULT_FONT_FAMILY,
    };

    let tilix_settings = get_tilix_settings()?;

    // Update the default Tilix profile, if set
    if let Some(capture) =
        Regex::new(r"(?s)\[profiles\].*default='(?P<id>.*?)'")?
            .captures(&tilix_settings)
    {
        let profile_id = capture.name("id").unwrap().as_str();

        let (font, size) = get_font_name_and_size(profile_id, &tilix_settings)
            .unwrap_or_else(|| (font, DEFAULT_FONT_SIZE));

        let settings = format!(
            "[profiles/{}]\nfont='{} {}'\nuse-system-font=false",
            profile_id, font, size
        );

        set_tilix_settings(&settings)
    }
    // Update the profile Tilix first created.
    else if tilix_settings
        .contains(&format!("[profiles/{}]\n", DEFAULT_PROFILE_ID))
    {
        let settings = format!(
            "[profiles/{}]\nfont='{} {}'\nuse-system-font=false",
            DEFAULT_PROFILE_ID, DEFAULT_FONT_FAMILY, DEFAULT_FONT_SIZE
        );

        set_tilix_settings(&settings)
    }
    // Manually create the Tilix tilix_settings settings.
    // Occurs if Tilix was installed, but never launched
    else {
        let settings = &format!(
            "[profiles/{}]\nvisible-name='Default'\nfont='{} {}'\nuse-system-font=false",
            DEFAULT_PROFILE_ID,
            font,
            DEFAULT_FONT_SIZE// TODO: change when setting.size is a feature
        );

        set_tilix_settings(&settings)
    }
}

// We should provide a utility function for this in app/mod.rs.
// Note that Windows would use ";" as a separator.
/// Validate programs in the PATH environment variable
fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        return path
            .split(":")
            .into_iter()
            .find(|p| fs::metadata(format!("{}/{}", p, program)).is_ok())
            .is_some();
    }

    false
}

/// Adjust/add new settings to Tilix's dconf entry.
/// A new entry is created if one does not exist.
fn set_tilix_settings(settings: &str) -> Result {
    let mut child = Command::new("dconf")
        .args(&["load", "/com/gexperts/Tilix/"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child = child.stdin.as_mut().ok_or(Box::new(Error::new(
        ErrorKind::Other,
        "Failed to capture dconf child process",
    )))?;

    match child.write_all(settings.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

/// Obtain a `String` of Tilix's settings inside dconf.
fn get_tilix_settings() -> Result<String> {
    let dconf = Command::new("dconf")
        .args(&["dump", "/com/gexperts/Tilix/"])
        .output()?
        .stdout;

    match std::str::from_utf8(&dconf) {
        Ok(s) => Ok(String::from(s)),
        Err(e) => Err(Box::new(e)),
    }
}

/// Obtain the font name and size in Tilix's dconf settings`.
fn get_font_name_and_size<'a>(
    profile_id: &str,
    dconf_settings: &'a str,
) -> Option<(&'a str, &'a str)> {
    let captures = Regex::new(&format!(
        r"(?s)\[profiles/{}\].*font='(?P<font>.*?)(?P<size>\d+)'",
        profile_id
    ))
    .unwrap()
    .captures(dconf_settings);

    match captures {
        Some(group) => {
            let font = group.name("font").unwrap().as_str();
            let size = group.name("size").unwrap().as_str();
            Some((font, size))
        }
        None => None,
    }
}
