/// Declares the `App` enum and uses the same `#[cfg]` for operations.
///
/// This allows for producing less code for platforms that don't support a given
/// app.
macro_rules! apps {
    ($(
        $(#[doc = $doc:literal])+
        $(#[cfg($cfg:meta)])?
        $name:ident = $option:expr,
    )+) => {
        /// A supported application.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum App {$(
            $(#[doc = $doc])+
            $(#[cfg($cfg)])?
            $name,
        )+}

        impl App {
            /// Parses the string into a known app.
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        $(#[cfg($cfg)])?
                        $option => Some(Self::$name),
                    )+
                    _ => None,
                }
            }

            /// Prints all app options.
            pub fn print_all_options() {
                $(
                    $(#[cfg($cfg)])?
                    println!($option);
                )+
            }
        }
    };
}

apps! {
    /// The "native" terminal app:
    ///
    /// - [macOS Terminal](https://en.wikipedia.org/wiki/Terminal_(macOS))
    /// - [Windows terminal](https://github.com/microsoft/terminal)
    /// - [GNOME terminal](https://gitlab.gnome.org/GNOME/gnome-terminal)
    Terminal = "terminal",

    /// macOS Xcode.app.
    #[cfg(target_os = "macos")]
    Xcode = "xcode",
}
