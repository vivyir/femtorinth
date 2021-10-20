#![doc = include_str!("../README.md")]

/// Module containing all data structures and their methods.
pub mod data_structures;
pub use data_structures::{Error, API_PREFIX};

mod api_functions;
pub use api_functions::{
    list_categories, list_game_versions, list_loaders, mod_get, search_mods, user_get, version_get,
    version_list,
};
