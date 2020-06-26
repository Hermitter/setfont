#[cfg(not(any(
    target_os = "macos",
    target_os = "linux",
    target_os = "windows",
)))]
compile_error!("Only macOS, Linux, and Windows are supported");

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::process;

pub mod app;
pub mod cli;
pub mod ext;
pub mod font;
pub mod shared;

use app::{App, Setting};
use ext::ArgMatchesExt;
use shared::Shared;

fn main() {
    let matches = cli::app().get_matches();

    match matches.subcommand() {
        ("list", _) => {
            App::print_all_options();
            return;
        }
        _ => {}
    };

    let app_args = matches.values_of_os("apps").expect("required");

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

    let setting = match (font, ligatures) {
        (Some(font), Some(ligatures)) => Setting::Both { font, ligatures },
        (Some(font), None) => Setting::Font(font),
        (None, Some(ligatures)) => Setting::Ligatures(ligatures),
        (None, None) => unreachable!("required"),
    };

    let mut did_error = false;
    let mut apps = Vec::<App>::new();

    for app_arg in app_args {
        let app = match app_arg.to_str() {
            Some(app) => match App::from_str(app) {
                Some(app) => app,
                None => {
                    eprintln!("error: unknown app {:?}", app);
                    did_error = true;
                    continue;
                }
            },
            None => {
                eprintln!("error: invalid UTF-8 string {:?}", app_arg);
                did_error = true;
                continue;
            }
        };

        apps.push(app);
    }

    let shared = Shared::new(did_error);

    // Remove duplicates.
    apps.sort_unstable();
    apps.dedup();

    // Avoid rayon overhead for 0 or 1 app.
    match apps.len() {
        0 => {}
        1 => {
            let app = apps[0];
            app.apply(&setting, &shared);
        }
        _ => {
            // Using slice to avoid extra overhead of draining the vector.
            let apps = apps.as_slice();

            apps.into_par_iter().for_each(|app| {
                app.apply(&setting, &shared);
            });
        }
    }

    if shared.did_error.into_inner() {
        process::exit(1);
    }
}
