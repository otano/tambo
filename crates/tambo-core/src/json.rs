use serde_json::Value;
use typst::foundations::{Dict, IntoValue};

pub fn escape_typst_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '$' => out.push_str("\\$"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

pub fn json_to_typst_value(val: &Value) -> typst::foundations::Value {
    match val {
        Value::Null => typst::foundations::Value::None,
        Value::Bool(b) => (*b).into_value(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_value()
            } else if let Some(f) = n.as_f64() {
                f.into_value()
            } else {
                typst::foundations::Value::None
            }
        }
        Value::String(s) => s.as_str().into_value(),
        Value::Array(arr) => {
            let items: Vec<typst::foundations::Value> =
                arr.iter().map(json_to_typst_value).collect();
            items.into_value()
        }
        Value::Object(map) => {
            let mut dict = Dict::new();
            for (k, v) in map {
                dict.insert(k.as_str().into(), json_to_typst_value(v));
            }
            dict.into_value()
        }
    }
}

pub fn json_to_typst_literal(val: &Value) -> String {
    match val {
        Value::Null => "none".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(f) = n.as_f64() {
                f.to_string()
            } else {
                "none".to_string()
            }
        }
        Value::String(s) => escape_typst_string(s),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_typst_literal).collect();
            format!("({})", items.join(", "))
        }
        Value::Object(map) => {
            let mut fields = Vec::with_capacity(map.len());
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for k in keys {
                let v = json_to_typst_literal(&map[k]);
                if k.chars().all(|c| c.is_alphanumeric() || c == '_')
                    && !k.starts_with(|c: char| c.is_ascii_digit())
                {
                    fields.push(format!("{}: {}", k, v));
                } else {
                    fields.push(format!("\"{}\": {}", k, v));
                }
            }
            format!("({})", fields.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_to_typst_value_string() {
        let v = json_to_typst_value(&json!("hello"));
        let s = format!("{:?}", v);
        assert!(s.contains("hello"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_null() {
        let v = json_to_typst_value(&Value::Null);
        assert_eq!(format!("{:?}", v), "None");
    }

    #[test]
    fn test_json_to_typst_value_integer() {
        let v = json_to_typst_value(&json!(42));
        let s = format!("{:?}", v);
        assert!(s.contains("42"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_bool() {
        let v = json_to_typst_value(&json!(true));
        let s = format!("{:?}", v);
        assert!(s.contains("true"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_array() {
        let v = json_to_typst_value(&json!(["a", "b"]));
        let s = format!("{:?}", v);
        assert!(s.contains('a'), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_value_object() {
        let v = json_to_typst_value(&json!({"key": "val"}));
        let s = format!("{:?}", v);
        assert!(s.contains("val"), "got {s:?}");
    }

    #[test]
    fn test_json_to_typst_literal_string() {
        assert_eq!(json_to_typst_literal(&json!("hello")), r#""hello""#);
    }

    #[test]
    fn test_json_to_typst_literal_null() {
        assert_eq!(json_to_typst_literal(&Value::Null), "none");
    }

    #[test]
    fn test_json_to_typst_literal_integer() {
        assert_eq!(json_to_typst_literal(&json!(42)), "42");
    }

    #[test]
    fn test_json_to_typst_literal_bool() {
        assert_eq!(json_to_typst_literal(&json!(true)), "true");
    }

    #[test]
    fn test_json_to_typst_literal_array() {
        assert_eq!(json_to_typst_literal(&json!(["a", "b"])), r#"("a", "b")"#);
    }

    #[test]
    fn test_json_to_typst_literal_object() {
        let result = json_to_typst_literal(&json!({"key": "val"}));
        assert_eq!(result, r#"(key: "val")"#);
    }

    #[test]
    fn test_escape_typst_string_basic() {
        assert_eq!(escape_typst_string("hello"), r#""hello""#);
    }

    #[test]
    fn test_escape_typst_string_special_chars() {
        assert_eq!(
            escape_typst_string("say \"hi\" $5"),
            r#""say \"hi\" \$5""#
        );
    }
}
