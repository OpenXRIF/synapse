use std::collections::HashMap;

use crate::prompting::rag::v1::waypoints::get_waypoints_list;

pub struct RagFiller<'a> {
    pub functions: HashMap<&'a str, fn(&str) -> String>,
}

impl<'a> RagFiller<'a> {
    pub fn new() -> Self {
        let mut functions: HashMap<&str, fn(&str) -> String> = HashMap::new();
        functions.insert("get_waypoints", get_waypoints_list);
        RagFiller { functions }
    }

    pub fn fill(&self, request: &str) -> String {
        let mut filled_request: String = request.to_string();

        for (key, value) in &self.functions {
            if request.starts_with(key) {
                filled_request = value(&request[key.len()..]);
            }
        }

        format!("{}", filled_request)
    }

    pub fn new_test(functions: HashMap<&'a str, fn(&str) -> String>) -> Self {
        RagFiller {
            functions: functions,
        }
    }
}
