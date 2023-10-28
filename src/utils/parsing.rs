use std::collections::HashMap;

use serde_json::Value;

pub fn parse_to_hashmap(input: &str) -> Result<HashMap<String, String>, &'static str> {
    let parsed: Result<Value, _> = serde_json::from_str(input);

    match parsed {
        Ok(data) => {
            if let Value::Object(obj) = data {
                let mut map = HashMap::new();
                for (key, value) in obj {
                    if let Value::String(value_str) = value {
                        map.insert(key, value_str);
                    } else {
                        return Err("Invalid value type");
                    }
                }
                Ok(map)
            } else {
                Err("Input is not a JSON object")
            }
        }
        Err(_) => Err("Failed to parse JSON"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let input = r#"{"keya":"valuea","keyb":"valueb"}"#;
        let result = parse_to_hashmap(input);
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.get("keya"), Some(&"valuea".to_string()));
        assert_eq!(map.get("keyb"), Some(&"valueb".to_string()));
    }

    #[test]
    fn test_funky_input() {
        let input = r#"{"keya":"13280fqcjascsl\";cz.aS}|w14981,42151c","keyb":"valueb"}"#;
        let result = parse_to_hashmap(input);
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.get("keya"), Some(&"13280fqcjascsl\";cz.aS}|w14981,42151c".to_string()));
        assert_eq!(map.get("keyb"), Some(&"valueb".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        let input = r#"{"keya":1,"keyb":"valueb"}"#; // Value of keya is not a string
        let result = parse_to_hashmap(input);
        assert!(result.is_err());

        let error_message = result.unwrap_err();
        assert_eq!(error_message, "Invalid value type");
    }
}
