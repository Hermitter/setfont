pub mod cli;

fn main() {
    let matches = cli::app().get_matches();

    let font = matches.value_of_os("font");
    let apps = matches.values_of_os("apps");

    let (font, apps) = match (font, apps) {
        (Some(font), Some(apps)) => (font, apps),
        _ => unreachable!("required values not provided"),
    };

    for app in apps {
        // TODO: Actually set the font.
        println!("Setting font {:?} for {:?}...", font, app);
    }
}
