#[cfg(not(any(
    target_os = "macos",
    target_os = "linux",
    target_os = "windows",
)))]
compile_error!("Only macOS, Linux, and Windows are supported");

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    process,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod app;
pub mod cli;
pub mod ext;
pub mod font;
pub mod shared;

use app::{App, Setting};
use ext::ArgMatchesExt;
use shared::Shared;

/// The catch-all error type of this program.
pub type Error = Box<dyn std::error::Error>;

pub type Result<T = ()> = std::result::Result<T, Error>;

fn main() {
    let matches = cli::app().get_matches();

    match matches.subcommand() {
        ("list", _) => {
            App::print_all_options();
            return;
        }
        _ => {}
    };

    let app_args = matches
        .values_of_os("apps")
        .unwrap_or_else(|| unreachable!("required"));

    let font = matches.value_of_os("font").map(|font| {
        match font::Font::from_os_str(font) {
            Some(font) => font,
            None => {
                eprintln!("error: invalid font name {:?}", font);
                process::exit(1);
            }
        }
    });

    let ligatures = matches.ligatures_flag();

    let setting = Setting::new(font, ligatures)
        .unwrap_or_else(|| unreachable!("required"));

    // Indicates whether an error occurred at any point. If this is `true` by
    // the end of `main`, the process will terminate with a non-zero exit code.
    //
    // This is an atomic value so that it can later be (safely) mutated by more
    // than one thread in the case of multiple apps.
    let mut did_error = AtomicBool::new(false);

    let mut apps = Vec::<App>::new();

    for app_arg in app_args {
        let app = match app_arg.to_str() {
            Some(app) => match App::from_str(app) {
                Some(app) => app,
                None => {
                    eprintln!("error: unknown app {:?}", app);
                    *did_error.get_mut() = true;
                    continue;
                }
            },
            None => {
                eprintln!("error: invalid UTF-8 string {:?}", app_arg);
                *did_error.get_mut() = true;
                continue;
            }
        };

        apps.push(app);
    }

    let shared = Shared::new();

    // Remove duplicates.
    apps.sort_unstable();
    apps.dedup();

    // Avoid rayon overhead for 0 or 1 app.
    match apps.len() {
        0 => {}
        1 => {
            let app = apps[0];
            if let Err(error) = app.apply(&setting, &shared) {
                *did_error.get_mut() = true;
                eprintln!("error: {}", error);
            }
        }
        _ => {
            // Using slice to avoid extra overhead of draining the vector.
            let apps = apps.as_slice();

            apps.into_par_iter().for_each(|app| {
                if let Err(error) = app.apply(&setting, &shared) {
                    did_error.store(true, Ordering::SeqCst);
                    eprintln!("error: {}", error);
                }
            });
        }
    }

    if did_error.into_inner() {
        process::exit(1);
    }
}
