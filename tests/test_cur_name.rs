#![allow(deprecated)]
use wp_data_fmt::{Json, KeyValue, ProtoTxt, Raw, RecordFormatter, ValueFormatter};
use wp_model_core::model::types::value::ObjectValue;
use wp_model_core::model::{DataField, DataRecord, FieldStorage};

/// Test whether cur_name is used in nested Object formatting
#[test]
fn test_object_nested_cur_name() {
    // Create a field with original name and set cur_name
    let mut field = FieldStorage::from_owned(DataField::from_digit("original_name", 42));
    field.set_name("cur_name_value");

    // Verify get_name() returns cur_name
    assert_eq!(field.get_name(), "cur_name_value");

    // Insert into ObjectValue with a different key
    let mut obj = ObjectValue::new();
    obj.insert("object_key", field);

    // Format as JSON
    let json_fmt = Json;
    let json_output = json_fmt.format_value(&wp_model_core::model::Value::Obj(obj.clone()));
    println!("JSON output: {}", json_output);

    // Format as KeyValue
    let kv_fmt = KeyValue::new();
    let kv_output = kv_fmt.format_value(&wp_model_core::model::Value::Obj(obj.clone()));
    println!("KV output: {}", kv_output);

    // Format as ProtoTxt
    let proto_fmt = ProtoTxt::new();
    let proto_output = proto_fmt.format_value(&wp_model_core::model::Value::Obj(obj.clone()));
    println!("ProtoTxt output: {}", proto_output);

    // After fix: uses field.get_name() which returns cur_name
    assert!(json_output.contains("\"cur_name_value\":42"));
    assert!(kv_output.contains("\"cur_name_value\": 42"));
    assert!(proto_output.contains("cur_name_value: 42"));
}

/// Test whether cur_name is used in top-level record formatting
#[test]
fn test_record_top_level_cur_name() {
    // Create a field and set cur_name
    let mut field = FieldStorage::from_owned(DataField::from_digit("original_name", 42));
    field.set_name("display_name");

    let record = DataRecord {
        id: Default::default(),
        items: vec![field],
    };

    // Format as JSON
    let json_fmt = Json;
    let json_output = json_fmt.fmt_record(&record);
    println!("Record JSON output: {}", json_output);

    // Top-level uses field.get_name(), so should use cur_name
    assert!(json_output.contains("\"display_name\":42"));
}

/// Test Array formatting - should it use field names?
#[test]
fn test_array_formatting() {
    let field1 = FieldStorage::from_owned(DataField::from_digit("field1", 1));
    let mut field2 = FieldStorage::from_owned(DataField::from_digit("field2", 2));
    field2.set_name("renamed_field2");

    let arr_value = wp_model_core::model::Value::Array(vec![field1, field2]);

    // Format as JSON
    let json_fmt = Json;
    let json_output = json_fmt.format_value(&arr_value);
    println!("Array JSON output: {}", json_output);

    // Arrays typically don't include field names, just values
    assert_eq!(json_output, "[1,2]");
}

/// Test deeply nested Object with cur_name
#[test]
fn test_deeply_nested_object_cur_name() {
    // Create inner object
    let mut inner_field = FieldStorage::from_owned(DataField::from_digit("inner_original", 100));
    inner_field.set_name("inner_display");

    let mut inner_obj = ObjectValue::new();
    inner_obj.insert("inner_key", inner_field);

    // Create outer object containing inner object
    let mut outer_field = FieldStorage::from_owned(DataField::from_obj("outer_original", inner_obj));
    outer_field.set_name("outer_display");

    let mut outer_obj = ObjectValue::new();
    outer_obj.insert("outer_key", outer_field);

    // Format as JSON
    let json_fmt = Json;
    let json_output = json_fmt.format_value(&wp_model_core::model::Value::Obj(outer_obj));
    println!("Nested JSON output: {}", json_output);

    // Should use cur_name at all levels
    assert!(json_output.contains("\"outer_display\""));
    assert!(json_output.contains("\"inner_display\""));
    assert!(json_output.contains("100"));
}

/// Test mixed scenario: record with nested object using cur_name
#[test]
fn test_record_with_nested_object_cur_name() {
    // Create nested object field
    let mut nested_field = FieldStorage::from_owned(DataField::from_digit("nested_original", 99));
    nested_field.set_name("nested_renamed");

    let mut obj = ObjectValue::new();
    obj.insert("obj_key", nested_field);

    // Create top-level field with cur_name
    let mut top_field = FieldStorage::from_owned(DataField::from_obj("top_original", obj));
    top_field.set_name("top_renamed");

    let record = DataRecord {
        id: Default::default(),
        items: vec![top_field],
    };

    // Format with different formatters
    let json_fmt = Json;
    let json_output = json_fmt.fmt_record(&record);
    println!("Mixed JSON output: {}", json_output);

    let kv_fmt = KeyValue::new();
    let kv_output = kv_fmt.fmt_record(&record);
    println!("Mixed KV output: {}", kv_output);

    let raw_fmt = Raw::new();
    let raw_output = raw_fmt.fmt_record(&record);
    println!("Mixed Raw output: {}", raw_output);

    // All should use cur_name at both levels
    assert!(json_output.contains("\"top_renamed\""));
    assert!(json_output.contains("\"nested_renamed\""));
    assert!(kv_output.contains("top_renamed"));
    assert!(kv_output.contains("nested_renamed"));
    assert!(raw_output.contains("nested_renamed"));
}

