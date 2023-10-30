use serde_json::Value as JsonValue;

pub fn set_table_row_tags(tags: &Option<JsonValue>) -> Option<Vec<String>> {
    match tags {
        Some(JsonValue::Array(arr)) => {
            let string_tags: Vec<String> = arr.iter()
                .filter_map(|val| val.as_str())
                .map(|s| s.to_string())
                .collect();
            Some(string_tags)
        },
        _ => None,
    }
}

