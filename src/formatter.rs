use wp_model_core::model::data::record::RecordItem;
use wp_model_core::model::{DataRecord, FieldStorage, Value, types::value::ObjectValue};

use crate::{FormatType, SqlFormat};

// ============================================================================
// 新设计：简化统一的 Formatter trait
// ============================================================================

/// 核心 trait：值格式化器
///
/// 只需实现一个方法来格式化任意 Value。这是最核心、最简单的接口。
pub trait ValueFormatter {
    type Output;

    /// 格式化任意值
    fn format_value(&self, value: &Value) -> Self::Output;
}

/// 扩展 trait：记录格式化器
///
/// 在 ValueFormatter 基础上，提供格式化字段和记录的能力。
/// 大部分实现都可以使用默认实现。
pub trait RecordFormatter: ValueFormatter {
    /// 格式化单个字段
    ///
    /// 默认实现：只格式化字段的值，忽略字段名
    fn fmt_field(&self, field: &FieldStorage) -> Self::Output {
        self.format_value(field.get_value())
    }

    /// 格式化整条记录
    ///
    /// 这个方法需要具体实现，因为不同格式对记录的表示方式差异很大：
    /// - JSON: `{"name":"Alice","age":30}`
    /// - CSV: `Alice,30`
    /// - KV: `name: "Alice", age: 30`
    fn fmt_record(&self, record: &DataRecord) -> Self::Output;
}

// ============================================================================
// 旧设计：保持向后兼容（标记为 deprecated）
// ============================================================================

/// 旧的 DataFormat trait
///
/// **已弃用**: 请使用 `ValueFormatter` 和 `RecordFormatter` 替代。
///
/// 这个 trait 将在下一个主要版本中移除。
#[deprecated(
    since = "0.2.0",
    note = "Use ValueFormatter and RecordFormatter instead. This trait will be removed in the next major version."
)]
pub trait DataFormat {
    type Output;

    fn format_null(&self) -> Self::Output;
    fn format_bool(&self, value: &bool) -> Self::Output;
    fn format_string(&self, value: &str) -> Self::Output;
    fn format_i64(&self, value: &i64) -> Self::Output;
    fn format_f64(&self, value: &f64) -> Self::Output;
    fn format_ip(&self, value: &std::net::IpAddr) -> Self::Output;
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> Self::Output;

    fn format_object(&self, value: &ObjectValue) -> Self::Output;
    fn format_array(&self, value: &[FieldStorage]) -> Self::Output; // ← 改为 FieldStorage

    fn fmt_value(&self, value: &Value) -> Self::Output {
        match value {
            Value::Null => self.format_null(),
            Value::Bool(v) => self.format_bool(v),
            Value::Chars(v) => self.format_string(v),
            Value::Digit(v) => self.format_i64(v),
            Value::Float(v) => self.format_f64(v),
            Value::IpAddr(v) => self.format_ip(v),
            Value::Time(v) => self.format_datetime(v),
            Value::Obj(v) => self.format_object(v),
            Value::Array(v) => self.format_array(v), // v 现在是 &Vec<FieldStorage>
            _ => self.format_string(&value.to_string()),
        }
    }

    fn format_field(&self, field: &FieldStorage) -> Self::Output;
    fn format_record(&self, record: &DataRecord) -> Self::Output;
}

/// 旧的 StaticDataFormatter trait
///
/// **已弃用**: 请使用 `ValueFormatter` 替代（使用实例而不是静态方法）。
///
/// 这个 trait 将在下一个主要版本中移除。
#[deprecated(
    since = "0.2.0",
    note = "Use ValueFormatter instead with instance methods. This trait will be removed in the next major version."
)]
pub trait StaticDataFormatter {
    type Output;

    fn stdfmt_null() -> Self::Output;
    fn stdfmt_bool(value: &bool) -> Self::Output;
    fn stdfmt_string(value: &str) -> Self::Output;
    fn stdfmt_i64(value: &i64) -> Self::Output;
    fn stdfmt_f64(value: &f64) -> Self::Output;
    fn stdfmt_ip_addr(value: &std::net::IpAddr) -> Self::Output;
    fn stdfmt_datetime(value: &chrono::NaiveDateTime) -> Self::Output;

    fn stdfmt_object(value: &ObjectValue) -> Self::Output;
    fn stdfmt_array(value: &[FieldStorage]) -> Self::Output; // ← 改为 FieldStorage

    fn stdfmt_value(value: &Value) -> Self::Output {
        match value {
            Value::Null => Self::stdfmt_null(),
            Value::Bool(v) => Self::stdfmt_bool(v),
            Value::Chars(v) => Self::stdfmt_string(v),
            Value::Digit(v) => Self::stdfmt_i64(v),
            Value::Float(v) => Self::stdfmt_f64(v),
            Value::IpAddr(v) => Self::stdfmt_ip_addr(v),
            Value::Time(v) => Self::stdfmt_datetime(v),
            Value::Obj(v) => Self::stdfmt_object(v),
            Value::Array(v) => Self::stdfmt_array(v), // v 现在是 &Vec<FieldStorage>
            _ => Self::stdfmt_string(&value.to_string()),
        }
    }

    fn stdfmt_field(field: &FieldStorage) -> Self::Output;
    fn stdfmt_record(record: &DataRecord) -> Self::Output;
}

#[allow(deprecated)]
trait AsDataFormatter {
    fn as_formatter(&self) -> &dyn DataFormat<Output = String>;
}

#[allow(deprecated)]
impl AsDataFormatter for FormatType {
    fn as_formatter(&self) -> &dyn DataFormat<Output = String> {
        match self {
            FormatType::Csv(f) => f,
            FormatType::Json(f) => f,
            FormatType::Kv(f) => f,
            FormatType::Sql(f) => f,
            FormatType::Raw(f) => f,
            FormatType::ProtoText(f) => f,
        }
    }
}

#[allow(deprecated)]
impl AsDataFormatter for SqlFormat {
    fn as_formatter(&self) -> &dyn DataFormat<Output = String> {
        match self {
            SqlFormat::Json(f) => f,
            SqlFormat::Kv(f) => f,
            SqlFormat::Raw(f) => f,
            SqlFormat::ProtoText(f) => f,
        }
    }
}

#[allow(deprecated)]
impl DataFormat for FormatType {
    type Output = String;
    fn format_null(&self) -> Self::Output {
        self.as_formatter().format_null()
    }
    fn format_bool(&self, value: &bool) -> Self::Output {
        self.as_formatter().format_bool(value)
    }
    fn format_string(&self, value: &str) -> Self::Output {
        self.as_formatter().format_string(value)
    }
    fn format_i64(&self, value: &i64) -> Self::Output {
        self.as_formatter().format_i64(value)
    }
    fn format_f64(&self, value: &f64) -> Self::Output {
        self.as_formatter().format_f64(value)
    }
    fn format_ip(&self, value: &std::net::IpAddr) -> Self::Output {
        self.as_formatter().format_ip(value)
    }
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> Self::Output {
        self.as_formatter().format_datetime(value)
    }
    fn format_object(&self, value: &ObjectValue) -> Self::Output {
        self.as_formatter().format_object(value)
    }
    fn format_array(&self, value: &[FieldStorage]) -> Self::Output {
        self.as_formatter().format_array(value)
    }
    fn format_field(&self, field: &FieldStorage) -> Self::Output {
        self.as_formatter().format_field(field)
    }
    fn format_record(&self, record: &DataRecord) -> Self::Output {
        self.as_formatter().format_record(record)
    }
}

// ============================================================================
// 新 trait 实现：FormatType 和 SqlFormat
// ============================================================================

impl ValueFormatter for FormatType {
    type Output = String;

    fn format_value(&self, value: &Value) -> String {
        match self {
            FormatType::Csv(f) => f.format_value(value),
            FormatType::Json(f) => f.format_value(value),
            FormatType::Kv(f) => f.format_value(value),
            FormatType::Sql(f) => f.format_value(value),
            FormatType::Raw(f) => f.format_value(value),
            FormatType::ProtoText(f) => f.format_value(value),
        }
    }
}

impl RecordFormatter for FormatType {
    fn fmt_field(&self, field: &FieldStorage) -> String {
        match self {
            FormatType::Csv(f) => f.fmt_field(field),
            FormatType::Json(f) => f.fmt_field(field),
            FormatType::Kv(f) => f.fmt_field(field),
            FormatType::Sql(f) => f.fmt_field(field),
            FormatType::Raw(f) => f.fmt_field(field),
            FormatType::ProtoText(f) => f.fmt_field(field),
        }
    }

    fn fmt_record(&self, record: &DataRecord) -> String {
        match self {
            FormatType::Csv(f) => f.fmt_record(record),
            FormatType::Json(f) => f.fmt_record(record),
            FormatType::Kv(f) => f.fmt_record(record),
            FormatType::Sql(f) => f.fmt_record(record),
            FormatType::Raw(f) => f.fmt_record(record),
            FormatType::ProtoText(f) => f.fmt_record(record),
        }
    }
}

impl ValueFormatter for SqlFormat {
    type Output = String;

    fn format_value(&self, value: &Value) -> String {
        match self {
            SqlFormat::Json(f) => f.format_value(value),
            SqlFormat::Kv(f) => f.format_value(value),
            SqlFormat::Raw(f) => f.format_value(value),
            SqlFormat::ProtoText(f) => f.format_value(value),
        }
    }
}

impl RecordFormatter for SqlFormat {
    fn fmt_field(&self, field: &FieldStorage) -> String {
        match self {
            SqlFormat::Json(f) => f.fmt_field(field),
            SqlFormat::Kv(f) => f.fmt_field(field),
            SqlFormat::Raw(f) => f.fmt_field(field),
            SqlFormat::ProtoText(f) => f.fmt_field(field),
        }
    }

    fn fmt_record(&self, record: &DataRecord) -> String {
        match self {
            SqlFormat::Json(f) => f.fmt_record(record),
            SqlFormat::Kv(f) => f.fmt_record(record),
            SqlFormat::Raw(f) => f.fmt_record(record),
            SqlFormat::ProtoText(f) => f.fmt_record(record),
        }
    }
}
