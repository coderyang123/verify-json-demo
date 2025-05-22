use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let path = "C://Users//141549//Downloads//a.json";
    let file = File::open(path).expect("无法打开文件");
    let mut reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8))
        .build(file);
    let mut data = String::new();
    reader.read_to_string(&mut data).expect("读取文件失败");
    let json: Value = serde_json::from_str(&data).expect("解析JSON失败");
    let arr = json.as_array().expect("不是数组");

    // 判断每个对象的字段是否包含null值
    check_null_values(&arr);

    // 判断给定的key列表，判断哪些对象的字段是无法转换成数值的
    let keys = vec!["tempCode", "tempCode2"];
    check_invalid_values(&arr, keys);

    let duration = start.elapsed();
    println!("耗时: {:?}", duration);
}

// 判断每个对象的字段是否包含null值
fn check_null_values(arr: &Vec<Value>) {
    for (i, obj) in arr.iter().enumerate() {
        if let Some(map) = obj.as_object() {
            let null_values: Vec<_> = map
                .iter()
                .filter(|(_, v)| v.is_null())
                .map(|(k, _)| k.clone())
                .collect();
            if !null_values.is_empty() {
                println!("第【{}】个对象的以下字段值为null：", i + 1);
                for k in null_values {
                    println!("键：【{}】", k);
                }
            }
        }
    }
}

// 判断给定的key列表，判断哪些对象的字段是无法转换成int64或者浮点数
fn check_invalid_values(arr: &Vec<Value>, keys: Vec<&str>) {
    for (i, obj) in arr.iter().enumerate() {
        if let Some(map) = obj.as_object() {
            let mut invalid_fields = Vec::new();
            for (k, v) in map.iter() {
                if keys.iter().any(|key| k.eq_ignore_ascii_case(key)) {
                    if let Some(s) = v.as_str() {
                        if s.parse::<f64>().is_err() {
                            invalid_fields.push((k.clone(), v.clone()));
                        }
                    }
                }
            }
            if !invalid_fields.is_empty() {
                println!("第【{}】个对象的以下字段无法转换成数值：", i + 1);
                for (k, v) in invalid_fields {
                    println!("键：【{}】，值：【{}】", k, v);
                }
            }
        }
    }
}
