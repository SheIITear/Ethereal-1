use std::collections::HashMap;

use crate::evaluation::object::Object;

pub mod util;
pub mod array;
pub mod fs;
/// Function to load a standard library 
/// # Arguments
/// * `lib` - The name of the library to load.
/// # Returns
/// `HashMap<String, Object>` - The environment with the library loaded.
pub fn get_std_lib(lib: String) -> Option<HashMap<String, Object>> {
    match lib.as_str() {
        "std:util" => Some(util::add_globals()),
        "std:array" => Some(array::add_globals()),
        "std:fs" => Some(fs::add_globals()),
        _ => None,
    }
}