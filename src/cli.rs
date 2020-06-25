use clap::{App, AppSettings, Arg};

const NAME: &str = env!("CARGO_PKG_NAME");

const AUTHOR: &str = "SavoryInk (https://savory.ink)";

const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

const ABOUT: &str = "
Sets the font of editors and terminals\
";

pub fn app() -> App<'static, 'static> {
    App::new(NAME)
        .author(AUTHOR)
        .version(VERSION)
        .about(ABOUT)
        .settings(&[AppSettings::ArgRequiredElseHelp])
        .arg(
            Arg::with_name("font")
                .help("The primary font to set")
                .value_name("FONT")
                .short("f")
                .long("font")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("apps")
                .help("The applications to set the primary font of")
                .value_name("APP")
                .short("a")
                .long("apps")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
}
