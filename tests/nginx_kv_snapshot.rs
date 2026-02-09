#![allow(deprecated)]
use chrono::NaiveDateTime;
use std::net::{IpAddr, Ipv4Addr};
use wp_data_fmt::{DataFormat, KeyValue};
use wp_model_core::model::{DataField, DataRecord, FieldStorage};

#[test]
fn nginx_access_log_kv_snapshot() {
    // Build a record mimicking parsed nginx access log fields
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
    let ts = NaiveDateTime::parse_from_str("2019-08-06 12:12:19", "%Y-%m-%d %H:%M:%S").unwrap();

    let record = DataRecord {
        id: Default::default(),
        items: vec![
            FieldStorage::from_owned(DataField::from_ip("ip", ip)),
            FieldStorage::from_owned(DataField::from_time("time", ts)),
            FieldStorage::from_owned(DataField::from_chars(
                "http/request",
                "GET /nginx-logo.png HTTP/1.1",
            )),
            FieldStorage::from_owned(DataField::from_digit("http/status", 200)),
            FieldStorage::from_owned(DataField::from_digit("length", 368)),
            FieldStorage::from_owned(DataField::from_chars("chars", "http://119.122.1.4/")),
            FieldStorage::from_owned(DataField::from_chars(
                "http/agent",
                "Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 ",
            )),
            FieldStorage::from_owned(DataField::from_chars("src_key", "_")),
        ],
    };

    let kv = KeyValue::new();
    let out = kv.format_record(&record);
    let expected = "ip: 192.168.1.2, time: 2019-08-06 12:12:19, http/request: \"GET /nginx-logo.png HTTP/1.1\", http/status: 200, length: 368, chars: \"http://119.122.1.4/\", http/agent: \"Mozilla/5.0(Macintosh; Intel Mac OS X 10_14_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 \", src_key: \"_\"";
    assert_eq!(out, expected);
}
