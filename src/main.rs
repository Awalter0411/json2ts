use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn json_file_to_ts(file_path: &str, interface_name: &str) -> String {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader).unwrap();
    let mut interface_string = String::from(format!("interface {} {{\n", interface_name));

    match data {
        Value::Object(map) => {
            for (key, value) in map {
                interface_string.push_str(&format!("\t{}: {};\n", key, convert_json_to_ts(&value)));
            }
        }
        _ => panic!("Root must be an object"),
    };

    interface_string.push_str("}");
    interface_string
}

fn convert_json_to_ts(json_value: &Value) -> String {
    match json_value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "any[]".to_string()
            } else {
                let mut vec_types: Vec<String> = vec![];
                let type_name = convert_json_to_ts(&arr[0]);
                vec_types.push(type_name);
                vec_types.join(" | ")
            }
        }
        Value::Object(map) => {
            let mut vec_properties: Vec<String> = vec![];
            for (key, value) in map {
                vec_properties.push(format!("{}: {}", key, convert_json_to_ts(&value)));
            }

            let mut interface_string = String::from("{");
            interface_string.push_str(&vec_properties.join(", "));
            interface_string.push_str("}");
            interface_string
        }
    }
}

fn main() {
    let file_path = "test.json";
    let interface_name = "Person";
    let ts_interface = json_file_to_ts(file_path, interface_name);
    println!("{}", ts_interface);
}
