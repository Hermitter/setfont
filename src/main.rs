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

use ext::ArgMatchesExt;

fn main() {
    let matches = cli::app().get_matches();

    match matches.subcommand() {
        ("list", _) => {
            app::App::print_all_options();
            return;
        }
        _ => {}
    };

    let font = matches.value_of_os("font");
    let ligatures = matches.ligatures_flag();

    let apps = matches.values_of_os("apps").expect("required");

    let font = font.map(|font| match font::Font::from_os_str(font) {
        Some(font) => font,
        None => {
            eprintln!("error: invalid font name {:?}", font);
            process::exit(1);
        }
    });

    for app in apps {
        let app = match app.to_str() {
            Some(app) => match app::App::from_str(app) {
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

        // TODO: Actually set the font and ligatures state.
        if let Some(font) = font {
            println!("Setting font {:?} for {:?}...", font.as_str(), app);
        }
        if let Some(ligatures) = ligatures {
            println!("Setting ligatures to {:?} for {:?}...", ligatures, app);
        }
    }
}
