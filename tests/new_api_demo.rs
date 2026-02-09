// 演示新 API 的使用示例
use wp_data_fmt::{Csv, Json, KeyValue, RecordFormatter, ValueFormatter};
use wp_model_core::model::{DataField, DataRecord, FieldStorage, Value};

#[test]
fn test_new_api_json() {
    let json = Json;

    // 测试 format_value
    let value = Value::Chars("hello".into());
    let result = json.format_value(&value);
    assert_eq!(result, "\"hello\"");

    // 测试 fmt_record
    let record = DataRecord {
        id: Default::default(),
        items: vec![
            FieldStorage::from_owned(DataField::from_chars("name", "Alice")),
            FieldStorage::from_owned(DataField::from_digit("age", 30)),
        ],
    };
    let result = json.fmt_record(&record);
    assert!(result.contains("\"name\":\"Alice\""));
    assert!(result.contains("\"age\":30"));
}

#[test]
fn test_new_api_csv() {
    let csv = Csv::default();

    // 测试 format_value
    let value = Value::Digit(42);
    let result = csv.format_value(&value);
    assert_eq!(result, "42");

    // 测试 fmt_record
    let record = DataRecord {
        id: Default::default(),
        items: vec![
            FieldStorage::from_owned(DataField::from_chars("a", "x")),
            FieldStorage::from_owned(DataField::from_digit("b", 10)),
        ],
    };
    let result = csv.fmt_record(&record);
    assert_eq!(result, "x,10");
}

#[test]
fn test_new_api_kv() {
    let kv = KeyValue::default();

    // 测试 format_value
    let value = Value::Bool(true);
    let result = kv.format_value(&value);
    assert_eq!(result, "true");

    // 测试 fmt_record
    let record = DataRecord {
        id: Default::default(),
        items: vec![FieldStorage::from_owned(DataField::from_chars("key", "value"))],
    };
    let result = kv.fmt_record(&record);
    assert!(result.contains("key"));
    assert!(result.contains("value"));
}

#[test]
fn test_new_api_array_formatting() {
    let json = Json;

    // 测试数组值的格式化
    let array_value = Value::Array(vec![
        FieldStorage::from_owned(DataField::from_digit("", 1)),
        FieldStorage::from_owned(DataField::from_digit("", 2)),
        FieldStorage::from_owned(DataField::from_digit("", 3)),
    ]);

    let result = json.format_value(&array_value);
    assert_eq!(result, "[1,2,3]");
}

#[test]
fn test_new_api_object_formatting() {
    let json = Json;

    // 测试对象值的格式化
    let mut obj = wp_model_core::model::types::value::ObjectValue::new();
    obj.insert(
        "x".to_string(),
        FieldStorage::from_owned(DataField::from_digit("x", 10)),
    );
    obj.insert(
        "y".to_string(),
        FieldStorage::from_owned(DataField::from_digit("y", 20)),
    );

    let obj_value = Value::Obj(obj);
    let result = json.format_value(&obj_value);
    assert!(result.contains("\"x\":10"));
    assert!(result.contains("\"y\":20"));
}
