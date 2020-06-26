#[cfg(not(any(
    target_os = "macos",
    target_os = "linux",
    target_os = "windows",
)))]
compile_error!("Only macOS, Linux, and Windows are supported");

use std::process;

pub mod app;
pub mod cli;
pub mod ext;
pub mod font;

use app::{App, Setting};
use ext::ArgMatchesExt;

fn main() {
    let matches = cli::app().get_matches();

    match matches.subcommand() {
        ("list", _) => {
            App::print_all_options();
            return;
        }
        _ => {}
    };

    let apps = matches.values_of_os("apps").expect("required");

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

    for app in apps {
        let app = match app.to_str() {
            Some(app) => match App::from_str(app) {
                Some(app) => app,
                None => {
                    eprintln!("error: unknown app {:?}", app);
                    process::exit(1);
                }
            },
            None => {
                eprintln!("error: invalid UTF-8 string {:?}", app);
                process::exit(1);
            }
        };

        app.apply(&setting);
    }
}
