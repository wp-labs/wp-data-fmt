#[allow(deprecated)]
use crate::formatter::DataFormat;
use crate::formatter::{RecordFormatter, ValueFormatter};
use wp_model_core::model::types::value::ObjectValue;
use wp_model_core::model::{DataRecord, DataType, FieldStorage, Value};

#[derive(Debug, Default)]
pub struct Raw;

impl Raw {
    pub fn new() -> Self {
        Self
    }
}

#[allow(deprecated)]
impl DataFormat for Raw {
    type Output = String;
    fn format_null(&self) -> String {
        String::new()
    }
    fn format_bool(&self, v: &bool) -> String {
        v.to_string()
    }
    fn format_string(&self, v: &str) -> String {
        v.to_string()
    }
    fn format_i64(&self, v: &i64) -> String {
        v.to_string()
    }
    fn format_f64(&self, v: &f64) -> String {
        v.to_string()
    }
    fn format_ip(&self, v: &std::net::IpAddr) -> String {
        v.to_string()
    }
    fn format_datetime(&self, v: &chrono::NaiveDateTime) -> String {
        v.to_string()
    }
    fn format_object(&self, value: &ObjectValue) -> String {
        if value.is_empty() {
            return "{}".to_string();
        }
        let segments: Vec<String> = value
            .iter()
            .map(|(k, v)| format!("{}={}", k, self.fmt_value(v.get_value())))
            .collect();
        format!("{{{}}}", segments.join(", "))
    }
    fn format_array(&self, value: &[FieldStorage]) -> String {
        if value.is_empty() {
            return "[]".to_string();
        }
        let content: Vec<String> = value
            .iter()
            .map(|field| self.fmt_value(field.get_value()))
            .collect();
        format!("[{}]", content.join(", "))
    }
    fn format_field(&self, field: &FieldStorage) -> String {
        match field.get_value() {
            Value::Chars(s) => s.to_string(),
            _ => self.fmt_value(field.get_value()),
        }
    }
    fn format_record(&self, record: &DataRecord) -> String {
        record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.format_field(f))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;
    use wp_model_core::model::DataField;

    #[test]
    fn test_raw_new() {
        let raw = Raw::new();
        assert_eq!(raw.format_null(), "");
    }

    #[test]
    fn test_raw_default() {
        let raw = Raw;
        assert_eq!(raw.format_null(), "");
    }

    #[test]
    fn test_format_null() {
        let raw = Raw;
        assert_eq!(raw.format_null(), "");
    }

    #[test]
    fn test_format_bool() {
        let raw = Raw;
        assert_eq!(raw.format_bool(&true), "true");
        assert_eq!(raw.format_bool(&false), "false");
    }

    #[test]
    fn test_format_string() {
        let raw = Raw;
        assert_eq!(raw.format_string("hello"), "hello");
        assert_eq!(raw.format_string("world"), "world");
        assert_eq!(raw.format_string(""), "");
    }

    #[test]
    fn test_format_i64() {
        let raw = Raw;
        assert_eq!(raw.format_i64(&0), "0");
        assert_eq!(raw.format_i64(&42), "42");
        assert_eq!(raw.format_i64(&-100), "-100");
        assert_eq!(raw.format_i64(&i64::MAX), i64::MAX.to_string());
    }

    #[test]
    fn test_format_f64() {
        let raw = Raw;
        assert_eq!(raw.format_f64(&3.24), "3.24");
        assert_eq!(raw.format_f64(&0.0), "0");
        assert_eq!(raw.format_f64(&-2.5), "-2.5");
    }

    #[test]
    fn test_format_ip() {
        let raw = Raw;
        let ipv4 = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(raw.format_ip(&ipv4), "192.168.1.1");

        let ipv6 = IpAddr::from_str("::1").unwrap();
        assert_eq!(raw.format_ip(&ipv6), "::1");
    }

    #[test]
    fn test_format_datetime() {
        let raw = Raw;
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = raw.format_datetime(&dt);
        assert!(result.contains("2024"));
        assert!(result.contains("01"));
        assert!(result.contains("15"));
    }

    #[test]
    fn test_format_field_chars() {
        let raw = Raw;
        let field = FieldStorage::from_owned(DataField::from_chars("name", "Alice"));
        let result = raw.format_field(&field);
        assert_eq!(result, "Alice");
    }

    #[test]
    fn test_format_field_digit() {
        let raw = Raw;
        let field = FieldStorage::from_owned(DataField::from_digit("age", 30));
        let result = raw.format_field(&field);
        assert_eq!(result, "30");
    }

    #[test]
    fn test_format_record() {
        let raw = Raw;
        let record = DataRecord {
            id: Default::default(),
            items: vec![
                FieldStorage::from_owned(DataField::from_chars("name", "Alice")),
                FieldStorage::from_owned(DataField::from_digit("age", 30)),
            ],
        };
        let result = raw.format_record(&record);
        assert_eq!(result, "Alice 30");
    }

    #[test]
    fn test_format_array_empty() {
        let raw = Raw;
        let arr: Vec<FieldStorage> = vec![];
        assert_eq!(raw.format_array(&arr), "[]");
    }

    #[test]
    fn test_format_array_with_values() {
        let raw = Raw;
        let arr = vec![
            FieldStorage::from_owned(DataField::from_digit("", 1)),
            FieldStorage::from_owned(DataField::from_digit("", 2)),
            FieldStorage::from_owned(DataField::from_digit("", 3)),
        ];
        let result = raw.format_array(&arr);
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn test_format_object_empty() {
        let raw = Raw;
        let obj = ObjectValue::new();
        assert_eq!(raw.format_object(&obj), "{}");
    }

    fn make_record_with_obj() -> DataRecord {
        let mut obj = ObjectValue::new();
        obj.insert(
            "ssl_cipher".to_string(),
            FieldStorage::from_owned(DataField::from_chars("ssl_cipher", "ECDHE")),
        );
        DataRecord {
            id: Default::default(),
            items: vec![
                FieldStorage::from_owned(DataField::from_digit("status", 200)),
                FieldStorage::from_owned(DataField::from_obj("extends", obj)),
                FieldStorage::from_owned(DataField::from_digit("length", 50)),
            ],
        }
    }

    #[test]
    fn test_format_record_with_obj_no_newlines() {
        let raw = Raw;
        let record = make_record_with_obj();
        let result = raw.format_record(&record);
        assert!(
            !result.contains('\n'),
            "record output should not contain newlines: {}",
            result
        );
        assert!(result.contains("ECDHE"));
    }

    #[test]
    fn test_fmt_record_with_obj_no_newlines() {
        let raw = Raw;
        let record = make_record_with_obj();
        let result = raw.fmt_record(&record);
        assert!(
            !result.contains('\n'),
            "record output should not contain newlines: {}",
            result
        );
    }

    #[test]
    fn test_old_new_api_consistency_nested() {
        let raw = Raw;
        let record = make_record_with_obj();
        assert_eq!(raw.format_record(&record), raw.fmt_record(&record));
    }
}

// ============================================================================
// 新 trait 实现：ValueFormatter + RecordFormatter
// ============================================================================

#[allow(clippy::items_after_test_module)]
impl ValueFormatter for Raw {
    type Output = String;

    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Null => String::new(),
            Value::Bool(v) => v.to_string(),
            Value::Chars(v) => v.to_string(),
            Value::Digit(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::IpAddr(v) => v.to_string(),
            Value::Time(v) => v.to_string(),
            Value::Obj(obj) => {
                if obj.is_empty() {
                    "{}".to_string()
                } else {
                    let segments: Vec<String> = obj
                        .iter()
                        .map(|(k, field)| format!("{}={}", k, self.format_value(field.get_value())))
                        .collect();
                    format!("{{{}}}", segments.join(", "))
                }
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    "[]".to_string()
                } else {
                    let content: Vec<String> = arr
                        .iter()
                        .map(|field| self.format_value(field.get_value()))
                        .collect();
                    format!("[{}]", content.join(", "))
                }
            }
            _ => value.to_string(),
        }
    }
}

impl RecordFormatter for Raw {
    fn fmt_field(&self, field: &FieldStorage) -> String {
        match field.get_value() {
            Value::Chars(s) => s.to_string(),
            _ => self.format_value(field.get_value()),
        }
    }

    fn fmt_record(&self, record: &DataRecord) -> String {
        record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.fmt_field(f))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
