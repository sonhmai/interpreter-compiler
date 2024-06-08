use std::collections::HashMap;
use std::iter::Map;
use crate::eval::Value;

/// Environment is a place to look up what value representation an identifier string
/// maps to for evaluation.
///
/// Example
///     let x = 5;
///     let y = x; // we want our evaluation to remember x is 5 here and assign 5 to y
pub struct Environment {
    map: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let map: HashMap<String, Value> = HashMap::new();

        Environment {
            map
        }
    }
    pub fn get(name: String) -> Value {
        Value::Null
    }

    pub fn set(name: String) -> () {
        ()
    }
}