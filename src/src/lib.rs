use std::collections::HashSet;
use serde_json::Map;
use serde_json::{Value, json};


#[derive(Debug, PartialEq)]
pub enum Difference {
    OnlyInFirst(String, Value),
    OnlyInSecond(String, Value),
    DifferentValues(String, Value, Value),
    NestedDifference(String, Vec<Difference>),
}

pub fn detect_differences(json1: &str, json2: &str) -> Vec<Difference> {
    // TODO refactor to avoid unnecesary json parsing, review tests
    let obj1: Value = serde_json::from_str(json1).unwrap_or_else(|_| Value::Null);
    let obj2: Value = serde_json::from_str(json2).unwrap_or_else(|_| Value::Null);

    fn diff_recursive(obj1: &Map<String, Value>, obj2: &Map<String, Value>) -> Vec<Difference> {
        let mut differences = Vec::new();
        let keys1: HashSet<_> = obj1.keys().collect();
        let keys2: HashSet<_> = obj2.keys().collect();

        for key in keys1.difference(&keys2) {
            differences.push(Difference::OnlyInFirst(format!("{}", key), obj1.get(*key).unwrap().clone()));
        }

        for key in keys2.difference(&keys1) {
            differences.push(Difference::OnlyInSecond(format!("{}", key), obj2.get(*key).unwrap().clone()));
        }

        for key in keys1.intersection(&keys2) {
            let val1 = obj1.get(*key).unwrap();
            let val2 = obj2.get(*key).unwrap();
            if val1 != val2 {
                if val1.is_object() && val2.is_object() {
                    let nested_diffs = diff_recursive(val1.as_object().unwrap(), val2.as_object().unwrap());
                    if !nested_diffs.is_empty() {
                        differences.push(Difference::NestedDifference((*key).clone(), nested_diffs));
                    }
                } else {
                    differences.push(Difference::DifferentValues(format!("{}", key), val1.clone(), val2.clone()));
                }
            }
        }

        differences
    }

    match (&obj1, &obj2) {
        (Value::Object(map1), Value::Object(map2)) => diff_recursive(map1, map2),
        _ => vec![Difference::DifferentValues("".to_string(), obj1.clone(), obj2.clone())],
    }
}


// Assuming the Difference enum and detect_differences function are defined as previously discussed
pub fn apply_differences(json: &mut serde_json::Value, differences: &[Difference]) {
    for difference in differences {
        match difference {
            Difference::OnlyInSecond(key, value) => {
                // For simplicity, assume top-level differences only; nested logic would follow similarly
                if let Value::Object(ref mut map) = json {
                    map.insert(key.clone(), value.clone());
                }
            },
            Difference::OnlyInFirst(key, _value) => {
                if let Value::Object(ref mut map) = json {
                    map.remove(key);
                }
            },
            Difference::DifferentValues(key, _old_value, new_value) => {
                if let Value::Object(ref mut map) = json {
                    map.insert(key.clone(), new_value.clone());
                }
            },
            Difference::NestedDifference(key, nested_diffs) => {
                if let Value::Object(ref mut map) = json {
                    // Directly access the nested object using the key and pass it to apply_differences
                    if let Some(nested_value) = map.get_mut(key) {
                        apply_differences(nested_value, nested_diffs);
                    } else {
                        // If the nested object doesn't exist, create it with the differences
                        let mut nested_obj = Value::Object(Map::new());
                        apply_differences(&mut nested_obj, nested_diffs);
                        map.insert(key.clone(), nested_obj);
                    }
                }
            },
        }
    }
}


pub fn merge_jsons(json1: &str, json2: &str) -> String {
    let obj1: Value = serde_json::from_str(json1).unwrap_or_else(|_| json!({}));
    let obj2: Value = serde_json::from_str(json2).unwrap_or_else(|_| json!({}));
    let merged = merge(obj1, obj2);
    serde_json::to_string(&merged).unwrap()
}

fn merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut a), Value::Object(b)) => {
            for (k, v) in b {
                let value = a.get(&k).cloned().unwrap_or(Value::Null);
                a.insert(k, merge(value, v));
            }
            Value::Object(a)
        },
        (_, b) => b,
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module.

    #[test]
    fn test_merge_simple() {
        let json1 = r#"{"key1": "value1"}"#;
        let json2 = r#"{"key2": "value2"}"#;
        let result = merge_jsons(json1, json2);
        let expected = r#"{"key1":"value1","key2":"value2"}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_overlapping_keys() {
        let json1 = r#"{"key": "value1"}"#;
        let json2 = r#"{"key": "value2"}"#;
        let result = merge_jsons(json1, json2);
        let expected = r#"{"key":"value2"}"#; // Expect json2's value to overwrite json1's.
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_nested() {
        let json1 = r#"{"nested": {"key1": "value1"}}"#;
        let json2 = r#"{"nested": {"key2": "value2"}}"#;
        let result = merge_jsons(json1, json2);
        let expected = r#"{"nested":{"key1":"value1","key2":"value2"}}"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_invalid_json() {
        let json1 = r#"{"key":"value1"}"#;
        let json2 = r#"I am not a valid JSON!"#;
        let result = merge_jsons(json1, json2);
        // Expect it to return the valid JSON unchanged if the second one is invalid.
        assert_eq!(result, json1);
    }
    #[test]
    fn main_test() {
        let mut json2 : Value = serde_json::from_str(r#"{
            "key1": "original_value1",
            "key2": "original_value2",
            "nested": {
                "key_common": "original_common_value",
                "unique_to_2": "value_unique_2"
            }
        }"#).unwrap();
        let mut json3 = json2.clone();
    
        let differences = vec![
            Difference::OnlyInSecond("new_key".to_string(), Value::String("new_value".to_string())),
            Difference::DifferentValues("key1".to_string(), Value::Null, Value::String("modified_value1".to_string())),
            Difference::NestedDifference("nested".to_string(), vec![
                Difference::DifferentValues("key_common".to_string(), Value::Null, Value::String("modified_common_value".to_string())),
            ]),
            Difference::NestedDifference("nested2".to_string(), vec![
                Difference::DifferentValues("key_common2".to_string(), Value::Null, Value::String("modified_common_value".to_string())),
            ])
        ];
        let expected : Value = serde_json::from_str(r#"{
            "key1": "modified_value1",
            "key2": "original_value2",
            "new_key": "new_value",
            "nested": {
                "key_common": "modified_common_value",
                "unique_to_2": "value_unique_2"
            },
            "nested2": {
                "key_common2": "modified_common_value"
            }
        }"#).unwrap();
        apply_differences(&mut json3, &differences);
        let result =serde_json::to_string_pretty(&json3).unwrap();
        assert_eq!(result, serde_json::to_string_pretty(&expected).unwrap());
        let differences1 = detect_differences(&result, &serde_json::to_string_pretty(&expected).unwrap());
        assert_eq!(differences1, vec![]);
        // Apply the differences to the original JSON to get the expected JSON
        let differences2 = detect_differences(&serde_json::to_string_pretty(&json2).unwrap(), &serde_json::to_string_pretty(&expected).unwrap());
        apply_differences(&mut json2, &differences2);
        let result2 =serde_json::to_string_pretty(&json2).unwrap();
        assert_eq!(result2, serde_json::to_string_pretty(&expected).unwrap());
    }
    
}
