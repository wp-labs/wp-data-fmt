#[allow(deprecated)]
use crate::formatter::DataFormat;
use crate::formatter::{RecordFormatter, ValueFormatter};
use wp_model_core::model::{DataRecord, DataType, FieldStorage, Value, types::value::ObjectValue};

#[derive(Default)]
pub struct ProtoTxt;

impl ProtoTxt {
    pub fn new() -> Self {
        Self
    }
}

#[allow(deprecated)]
impl DataFormat for ProtoTxt {
    type Output = String;
    fn format_null(&self) -> String {
        String::new()
    }
    fn format_bool(&self, v: &bool) -> String {
        v.to_string()
    }
    fn format_string(&self, v: &str) -> String {
        format!("\"{}\"", v.replace('"', "\\\""))
    }
    fn format_i64(&self, v: &i64) -> String {
        v.to_string()
    }
    fn format_f64(&self, v: &f64) -> String {
        v.to_string()
    }
    fn format_ip(&self, v: &std::net::IpAddr) -> String {
        self.format_string(&v.to_string())
    }
    fn format_datetime(&self, v: &chrono::NaiveDateTime) -> String {
        self.format_string(&v.to_string())
    }
    fn format_object(&self, value: &ObjectValue) -> String {
        let items: Vec<String> = value
            .iter()
            .map(|(k, v)| format!("{}: {}", k, self.fmt_value(v.get_value())))
            .collect();
        items.join(" ")
    }
    fn format_array(&self, value: &[FieldStorage]) -> String {
        let items: Vec<String> = value
            .iter()
            .map(|f| self.fmt_value(f.get_value()))
            .collect();
        format!("[{}]", items.join(", "))
    }
    fn format_field(&self, field: &FieldStorage) -> String {
        if *field.get_meta() == DataType::Ignore {
            String::new()
        } else {
            match field.get_value() {
                Value::Obj(_) | Value::Array(_) => format!(
                    "{}: {}",
                    field.get_name(),
                    self.fmt_value(field.get_value())
                ),
                _ => format!(
                    "{}: {}",
                    field.get_name(),
                    self.fmt_value(field.get_value())
                ),
            }
        }
    }
    fn format_record(&self, record: &DataRecord) -> String {
        let items = record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.format_field(f))
            .collect::<Vec<_>>();
        // 生成标准的 proto-text 格式：消息用花括号包围
        format!("{{ {} }}", items.join(" "))
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
    fn test_proto_new() {
        let proto = ProtoTxt::new();
        assert_eq!(proto.format_null(), "");
    }

    #[test]
    fn test_proto_default() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_null(), "");
    }

    #[test]
    fn test_format_null() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_null(), "");
    }

    #[test]
    fn test_format_bool() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_bool(&true), "true");
        assert_eq!(proto.format_bool(&false), "false");
    }

    #[test]
    fn test_format_string() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_string("hello"), "\"hello\"");
        assert_eq!(proto.format_string(""), "\"\"");
    }

    #[test]
    fn test_format_string_escape_quotes() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_string("say \"hi\""), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn test_format_i64() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_i64(&0), "0");
        assert_eq!(proto.format_i64(&42), "42");
        assert_eq!(proto.format_i64(&-100), "-100");
    }

    #[test]
    fn test_format_f64() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_f64(&3.24), "3.24");
        assert_eq!(proto.format_f64(&0.0), "0");
    }

    #[test]
    fn test_format_ip() {
        let proto = ProtoTxt;
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(proto.format_ip(&ip), "\"192.168.1.1\"");
    }

    #[test]
    fn test_format_datetime() {
        let proto = ProtoTxt;
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = proto.format_datetime(&dt);
        assert!(result.starts_with('"'));
        assert!(result.ends_with('"'));
        assert!(result.contains("2024"));
    }

    #[test]
    fn test_format_field() {
        let proto = ProtoTxt;
        let field = FieldStorage::from_owned(DataField::from_chars("name", "Alice"));
        let result = proto.format_field(&field);
        assert_eq!(result, "name: \"Alice\"");
    }

    #[test]
    fn test_format_field_digit() {
        let proto = ProtoTxt;
        let field = FieldStorage::from_owned(DataField::from_digit("age", 30));
        let result = proto.format_field(&field);
        assert_eq!(result, "age: 30");
    }

    #[test]
    fn test_format_record() {
        let proto = ProtoTxt;
        let record = DataRecord {
            id: Default::default(),
            items: vec![
                FieldStorage::from_owned(DataField::from_chars("name", "Alice")),
                FieldStorage::from_owned(DataField::from_digit("age", 30)),
            ],
        };
        let result = proto.format_record(&record);
        assert!(result.starts_with("{ "));
        assert!(result.ends_with(" }"));
        assert!(result.contains("name: \"Alice\""));
        assert!(result.contains("age: 30"));
    }

    #[test]
    fn test_format_array() {
        let proto = ProtoTxt;
        let arr = vec![
            FieldStorage::from_owned(DataField::from_digit("x", 1)),
            FieldStorage::from_owned(DataField::from_digit("y", 2)),
        ];
        let result = proto.format_array(&arr);
        assert!(result.starts_with('['));
        assert!(result.ends_with(']'));
    }

    /// 构造一个包含 Obj 和 Array 字段的 record，用于嵌套格式化测试
    fn make_record_with_nested() -> DataRecord {
        let mut obj = ObjectValue::new();
        obj.insert(
            "ssl_cipher".to_string(),
            FieldStorage::from_owned(DataField::from_chars("ssl_cipher", "ECDHE")),
        );
        obj.insert(
            "ssl_protocol".to_string(),
            FieldStorage::from_owned(DataField::from_chars("ssl_protocol", "TLSv1.3")),
        );
        let arr = vec![
            DataField::from_chars("", "foo"),
            DataField::from_digit("", 42),
        ];
        DataRecord {
            id: Default::default(),
            items: vec![
                FieldStorage::from_owned(DataField::from_digit("sent_bytes", 200)),
                FieldStorage::from_owned(DataField::from_obj("extends", obj)),
                FieldStorage::from_owned(DataField::from_arr("tags", arr)),
                FieldStorage::from_owned(DataField::from_digit("match_chars", 50)),
            ],
        }
    }

    /// 回归测试：format_value(Value::Obj) 曾使用 \n 作为分隔符，
    /// 嵌入 record 后导致输出中出现意外换行
    #[test]
    fn test_format_record_with_obj_no_newlines() {
        let proto = ProtoTxt;
        let record = make_record_with_nested();
        let result = proto.format_record(&record);
        assert!(
            !result.contains('\n'),
            "record output should not contain newlines: {}",
            result
        );
        assert!(result.contains("ssl_cipher: \"ECDHE\""));
        assert!(result.contains("ssl_protocol: \"TLSv1.3\""));
        assert!(result.contains("sent_bytes: 200"));
        assert!(result.contains("match_chars: 50"));
    }

    #[test]
    fn test_fmt_record_with_obj_no_newlines() {
        let proto = ProtoTxt;
        let record = make_record_with_nested();
        let result = proto.fmt_record(&record);
        assert!(
            !result.contains('\n'),
            "record output should not contain newlines: {}",
            result
        );
        assert!(result.contains("ssl_cipher: \"ECDHE\""));
        assert!(result.contains("ssl_protocol: \"TLSv1.3\""));
        assert!(result.contains("tags: [\"foo\", 42]"));
    }

    /// 新旧 API 对含嵌套类型的 record 输出一致性
    #[test]
    fn test_old_new_api_consistency_nested() {
        let proto = ProtoTxt;
        let record = make_record_with_nested();
        assert_eq!(proto.format_record(&record), proto.fmt_record(&record));
    }

    #[test]
    fn test_old_new_api_consistency_scalar() {
        let proto = ProtoTxt;
        let record = DataRecord {
            id: Default::default(),
            items: vec![
                FieldStorage::from_owned(DataField::from_chars("name", "Alice")),
                FieldStorage::from_owned(DataField::from_digit("age", 30)),
            ],
        };
        assert_eq!(proto.format_record(&record), proto.fmt_record(&record));
    }
}

// ============================================================================
// 新 trait 实现：ValueFormatter + RecordFormatter
// ============================================================================

#[allow(clippy::items_after_test_module)]
impl ValueFormatter for ProtoTxt {
    type Output = String;

    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Null => String::new(),
            Value::Bool(v) => v.to_string(),
            Value::Chars(v) => format!("\"{}\"", v.replace('"', "\\\"")),
            Value::Digit(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::IpAddr(v) => format!("\"{}\"", v),
            Value::Time(v) => format!("\"{}\"", v),
            Value::Obj(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, field)| format!("{}: {}", k, self.format_value(field.get_value())))
                    .collect();
                items.join(" ")
            }
            Value::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|field| self.format_value(field.get_value()))
                    .collect();
                format!("[{}]", items.join(", "))
            }
            _ => format!("\"{}\"", value.to_string().replace('"', "\\\"")),
        }
    }
}

impl RecordFormatter for ProtoTxt {
    fn fmt_field(&self, field: &FieldStorage) -> String {
        if *field.get_meta() == DataType::Ignore {
            String::new()
        } else {
            format!(
                "{}: {}",
                field.get_name(),
                self.format_value(field.get_value())
            )
        }
    }

    fn fmt_record(&self, record: &DataRecord) -> String {
        let items = record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.fmt_field(f))
            .collect::<Vec<_>>();
        format!("{{ {} }}", items.join(" "))
    }
}
