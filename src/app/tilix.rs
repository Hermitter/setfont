use super::Setting;
use crate::{font::Font, Result, Shared};
use regex::Regex;
use std::io::{Error, ErrorKind, Write};
use std::process::{Command, Stdio};
use std::{env, fs};

// Schema for Tilix dconf settings: https://github.com/gnunn1/tilix/blob/4178cf16f4b15f06b679fa05c0fa6fc8afd40999/source/gx/tilix/preferences.d#L315
/// Profile ID Tilix creates on first launch
const DEFAULT_PROFILE_ID: &str = "2b7c4080-0ddd-46c5-8f23-563fd3ba789d";
/// Tilix's default setting for font family
const DEFAULT_FONT: &str = "Monospace Regular";
/// Tilix's default setting for font size
const DEFAULT_FONT_SIZE: &str = "12";

pub fn apply(setting: &Setting, _shared: &Shared) -> Result {
    // Ensure Tilix & dconf are installed.
    if !is_program_in_path("tilix") || !is_program_in_path("dconf") {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Could not find Tilix installation",
        )));
    }

    let tilix_settings = get_tilix_settings()?;

    // Update the default Tilix profile, if set
    if let Some(capture) =
        Regex::new(r"(?s)\[profiles\].*default='(?P<id>.*?)'")?
            .captures(&tilix_settings)
    {
        let profile_id = capture.name("id").unwrap().as_str();

        // TODO: When feature is released, change_current size for cli arg value.
        let (current_font, current_size) =
            get_font_name_and_size(profile_id, &tilix_settings);

        set_tilix_settings(&format!(
            "[profiles/{}]\nfont='{} {}'\nuse-system-font=false",
            profile_id,
            unwrap_font(&setting.font(), current_font),
            current_size
        ))
    }
    // Update the profile Tilix first created.
    else if tilix_settings
        .contains(&format!("[profiles/{}]\n", DEFAULT_PROFILE_ID))
    {
        // TODO: When feature is released, change_current size for cli arg value.
        let (current_font, current_size) =
            get_font_name_and_size(DEFAULT_PROFILE_ID, &tilix_settings);

        set_tilix_settings(&format!(
            "[profiles/{}]\nfont='{} {}'\nuse-system-font=false",
            DEFAULT_PROFILE_ID,
            unwrap_font(&setting.font(), current_font),
            current_size
        ))
    }
    // Manually create Tilix's dconf settings.
    // Occurs if Tilix was installed, but never launched.
    else {
        set_tilix_settings(& &format!(
            "[profiles/{}]\nvisible-name='Default'\nfont='{} {}'\nuse-system-font=false",
            DEFAULT_PROFILE_ID,
            unwrap_font(&setting.font(), DEFAULT_FONT),
            DEFAULT_FONT_SIZE// TODO: change when setting.size is a feature
        ))
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

/// Return CLI font argument or a fallback option.
fn unwrap_font<'a>(font: &'a Option<&Font>, fallback_font: &'a str) -> &'a str {
    match font {
        Some(f) => f.as_str(),
        None => fallback_font,
    }
}

/// Adjust/add new settings to Tilix's dconf entry.
/// A new entry is created if none exist.
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
/// Dconf settings can be viewed with: `$ dconf dump /com/gexperts/Tilix/`
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

/// Obtain the current font name and size in Tilix's dconf settings`.
fn get_font_name_and_size<'a>(
    profile_id: &str,
    dconf_settings: &'a str,
) -> (&'a str, &'a str) {
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
            (font, size)
        }
        None => (DEFAULT_FONT, DEFAULT_FONT_SIZE),
    }
}
