/// Declares the `App` enum and uses the same `#[cfg]` for operations.
///
/// This allows for producing less code for platforms that don't support a given
/// app.
macro_rules! apps {
    ($(
        $(#[doc = $doc:literal])+
        #[cfg($cfg:meta)]
        $name:ident = $option:expr,
    )+) => {
        /// A supported application.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum App {$(
            $(#[doc = $doc])+
            #[cfg($cfg)]
            $name,
        )+}

        impl App {
            /// Parses the string into a known app.
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        #[cfg($cfg)]
                        $option => Some(Self::$name),
                    )+
                    _ => None,
                }
            }
        }
    };
}

apps! {
    /// Either macOS Terminal.app or [Windows terminal].
    ///
    /// [Windows terminal]: https://github.com/microsoft/terminal
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Terminal = "terminal",

    /// macOS Xcode.app.
    #[cfg(target_os = "macos")]
    Xcode = "xcode",
}
