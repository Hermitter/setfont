use crate::font::{Font, LigaturesFlag};

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

impl<'a> Setting<'a> {
    pub fn new(
        font: Option<Font<'a>>,
        ligatures: Option<LigaturesFlag>,
    ) -> Option<Self> {
        match (font, ligatures) {
            (Some(font), Some(ligatures)) => {
                Some(Self::Both { font, ligatures })
            }
            (Some(font), None) => Some(Self::Font(font)),
            (None, Some(ligatures)) => Some(Self::Ligatures(ligatures)),
            (None, None) => None,
        }
    }
}
