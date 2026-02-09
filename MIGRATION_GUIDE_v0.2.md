# wp-data-fmt v0.2.0 迁移指南

## 新特性：简化的 Formatter API

wp-data-fmt v0.2.0 引入了新的简化 API，大幅减少了实现格式化器所需的代码量。

### 新 Trait：ValueFormatter 和 RecordFormatter

#### 之前的设计

旧的 `DataFormat` trait 需要实现 12 个方法：

```rust
pub trait DataFormat {
    fn format_null(&self) -> Self::Output;
    fn format_bool(&self, value: &bool) -> Self::Output;
    fn format_string(&self, value: &str) -> Self::Output;
    fn format_i64(&self, value: &i64) -> Self::Output;
    fn format_f64(&self, value: &f64) -> Self::Output;
    fn format_ip(&self, value: &std::net::IpAddr) -> Self::Output;
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> Self::Output;
    fn format_object(&self, value: &ObjectValue) -> Self::Output;
    fn format_array(&self, value: &[FieldStorage]) -> Self::Output;
    fn fmt_value(&self, value: &Value) -> Self::Output;
    fn format_field(&self, field: &FieldStorage) -> Self::Output;
    fn format_record(&self, record: &DataRecord) -> Self::Output;
}
```

#### 现在的设计

新的设计只需要 **2 个方法**：

```rust
/// 核心 trait - 只需实现一个方法
pub trait ValueFormatter {
    type Output;

    /// 格式化任意值
    fn format_value(&self, value: &Value) -> Self::Output;
}

/// 扩展 trait - 增加记录格式化能力
pub trait RecordFormatter: ValueFormatter {
    /// 格式化字段 (有默认实现)
    fn fmt_field(&self, field: &FieldStorage) -> Self::Output {
        self.format_value(field.get_value())
    }

    /// 格式化记录
    fn fmt_record(&self, record: &DataRecord) -> Self::Output;
}
```

### 使用示例

#### 实现自定义格式化器

```rust
use wp_data_fmt::{ValueFormatter, RecordFormatter};
use wp_model_core::model::{Value, DataRecord, DataType, FieldStorage};
use wp_model_core::model::data::record::RecordItem;

struct MyFormatter;

impl ValueFormatter for MyFormatter {
    type Output = String;

    fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Null => "NULL".to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Chars(s) => format!("'{}'", s),
            Value::Digit(n) => n.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|field| self.format_value(field.get_value()))
                    .collect();
                format!("[{}]", items.join(", "))
            }
            Value::Obj(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, field)| {
                        format!("{}: {}", k, self.format_value(field.get_value()))
                    })
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            _ => value.to_string(),
        }
    }
}

impl RecordFormatter for MyFormatter {
    fn fmt_record(&self, record: &DataRecord) -> String {
        record.items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.fmt_field(f))
            .collect::<Vec<_>>()
            .join(" | ")
    }
}
```

#### 使用内置格式化器

```rust
use wp_data_fmt::{Json, RecordFormatter};
use wp_model_core::model::{DataRecord, DataField, FieldStorage};

fn main() {
    let formatter = Json;

    let record = DataRecord {
        id: Default::default(),
        items: vec![
            FieldStorage::from_owned(DataField::from_chars("name", "Alice")),
            FieldStorage::from_owned(DataField::from_digit("age", 30)),
        ],
    };

    // 使用新 API
    let result = formatter.fmt_record(&record);
    println!("{}", result);  // {"name":"Alice","age":30}
}
```

### 与旧 API 的对比

| 特性 | 旧 API (DataFormat) | 新 API (ValueFormatter + RecordFormatter) |
|------|---------------------|-------------------------------------------|
| 需要实现的方法 | 12 个 | 1-2 个 |
| 代码量 | ~200 行 | ~50 行 |
| 自动支持新类型 | ❌ 需要修改 trait | ✅ 只需修改 match |
| 方法名冲突 | ✅ 无冲突 | ✅ 使用 fmt_ 前缀避免 |

### 向后兼容性

旧的 `DataFormat` 和 `StaticDataFormatter` trait 仍然可用，但已标记为 `deprecated`。

它们将在下一个主要版本中移除。建议尽快迁移到新 API。

### 优势

1. **更简单**：只需实现 1-2 个方法，而不是 12 个
2. **更灵活**：format_value 内部可以自由组织代码
3. **更易维护**：代码量减少 60%
4. **自动扩展**：添加新类型时只需修改 match 语句

### 迁移建议

1. 新项目直接使用 `ValueFormatter` 和 `RecordFormatter`
2. 现有项目可以逐步迁移，新旧 API 可以共存
3. 计划在 v1.0.0 移除旧 API

## 变更日志

### v0.2.0 (2026-02-09)

#### 新增
- 新增 `ValueFormatter` trait：核心值格式化接口
- 新增 `RecordFormatter` trait：记录格式化扩展
- 所有内置格式化器实现新 trait：Json, Csv, KeyValue, ProtoTxt, Raw, SqlInsert

#### 弃用
- `DataFormat` trait 标记为 deprecated
- `StaticDataFormatter` trait 标记为 deprecated

#### 改进
- 格式化器实现代码量减少 60%
- 更易于实现自定义格式化器
- 更好的可扩展性

### v0.1.0 (之前)

- 初始版本
- 实现基础格式化器
- 适配 wp-model-core 0.8.x
