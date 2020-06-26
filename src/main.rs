use std::process;

pub mod app;
pub mod cli;
pub mod font;

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
    let apps = matches.values_of_os("apps");

    let (font, apps) = match (font, apps) {
        (Some(font), Some(apps)) => (font, apps),
        _ => unreachable!("required values not provided"),
    };

    let font = match font::Font::from_os_str(font) {
        Some(font) => font,
        None => {
            eprintln!("error: invalid font name {:?}", font);
            process::exit(1);
        }
    };

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

        // TODO: Actually set the font.
        println!("Setting font {:?} for {:?}...", font.as_str(), app);
    }
}
