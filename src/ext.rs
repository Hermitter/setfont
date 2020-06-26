use crate::font::LigaturesFlag;

/// An extension trait for `clap::ArgMatches`.
pub trait ArgMatchesExt<'a> {
    /// Returns the result of `--[no-]ligatures`.
    fn ligatures_flag(&self) -> Option<LigaturesFlag>;
}

impl<'a> ArgMatchesExt<'a> for clap::ArgMatches<'a> {
    fn ligatures_flag(&self) -> Option<LigaturesFlag> {
        if self.is_present("ligatures") {
            Some(LigaturesFlag::Enable)
        } else if self.is_present("no-ligatures") {
            Some(LigaturesFlag::Disable)
        } else {
            None
        }
    }
}
