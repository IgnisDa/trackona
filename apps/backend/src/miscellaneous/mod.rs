use enum_meta::{meta, Meta};
use strum::{Display, EnumIter};

use crate::traits::MediaProviderLanguages;

pub mod resolver;

#[derive(Display, EnumIter)]
pub enum DefaultCollection {
    Watchlist,
    #[strum(serialize = "In Progress")]
    InProgress,
    Done,
    Monitoring,
    Custom,
}

meta! {
    DefaultCollection, &'static str;
    Watchlist, "Things I want to watch in the future.";
    InProgress, "Media items that I am currently watching.";
    Done, "Media items that I have completed.";
    Monitoring, "Items that I am keeping an eye on.";
    Custom, "Items that I have created manually.";
}

#[derive(Debug, Clone)]
pub struct CustomService {}

impl MediaProviderLanguages for CustomService {
    fn supported_languages() -> Vec<String> {
        ["us"].into_iter().map(String::from).collect()
    }

    fn default_language() -> String {
        "us".to_owned()
    }
}
