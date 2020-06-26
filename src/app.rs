use crate::font::{Font, LigaturesFlag};

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
    /// [Alacritty](https://github.com/alacritty/alacritty) terminal.
    Alacritty = "alacritty",

    /// [Atom](https://github.com/atom/atom) editor by GitHub.
    Atom = "atom",

    /// [Sublime Text](https://www.sublimetext.com) editor.
    SublimeText = "sublimetext",

    /// The "native" terminal app:
    ///
    /// - [macOS Terminal](https://en.wikipedia.org/wiki/Terminal_(macOS))
    /// - [Windows terminal](https://github.com/microsoft/terminal)
    /// - [GNOME terminal](https://gitlab.gnome.org/GNOME/gnome-terminal)
    Terminal = "terminal",

    /// [Visual Studio Code](https://code.visualstudio.com) editor by Microsoft.
    VsCode = "vscode",

    /// macOS Xcode.app.
    #[cfg(target_os = "macos")]
    Xcode = "xcode",
}

impl App {
    /// Applies a setting state to the app, based on input arguments.
    pub fn apply(self, setting: &Setting) {
        unimplemented!("apply {:?} to {:?}", setting, self);
    }
}

/// How to make changes to an app.
///
/// The state of having neither a font nor a ligatures flag is not possible.
/// This type makes that state unrepresentable, avoiding needing to do awkward
/// error handling for a state that can't happen.
#[derive(Clone, Copy, Debug)]
pub enum Setting<'a> {
    /// Only set the font.
    Font(Font<'a>),
    /// Only set orthographic ligatures.
    Ligatures(LigaturesFlag),
    /// Set both the font and orthographic ligatures.
    Both {
        font: Font<'a>,
        ligatures: LigaturesFlag,
    },
}
