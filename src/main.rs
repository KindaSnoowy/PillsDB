use std::{collections::HashMap, io, str, u8};

#[derive(PartialEq, Debug)]
enum DataType {
    String  = 0,
    Int     = 1,
    Float   = 2,
    Bool    = 3,
}

#[derive(Debug)]
struct DbValue {
    typetag: DataType,
    data: Vec<u8>,
}


impl DbValue {

// == To set data types easily. ==

    fn from_str(s: &str) -> Self {
        DbValue {
            typetag: DataType::String,
            data: s.as_bytes().to_vec(),
        }
    }

    fn from_i64(i: i64) -> Self {
        DbValue {
            typetag: DataType::Int,
            data: i.to_string().as_bytes().to_vec(),
        }
    }

    fn from_f64(f: f64) -> Self {
        DbValue {
            typetag: DataType::Float,
            data: f.to_string().as_bytes().to_vec(),
        }
    }

    fn from_bool(b: bool) -> Self {
        DbValue {
            typetag: DataType::Bool,
            data: if b { "true" } else { "false" }.as_bytes().to_vec(),
        }
    }

// == To get data types easily. ==

    fn as_string(&self) -> Option<&str> {
        if self.typetag == DataType::String {
            Some(str::from_utf8(&self.data).unwrap())
        } else {
            None
        }
    }

    fn as_int(&self) -> Option<i64> {
        if self.typetag == DataType::Int && self.data.len() == 8 { // i64 and f64 are always 8 bytes.
            Some(i64::from_ne_bytes(self.data[..8].try_into().unwrap()))
        } else {
            None
        }
    }

    fn as_float(&self) -> Option<f64> {
        if self.typetag == DataType::Float && self.data.len() == 8 { // i64 and f64 are always 8 bytes.
            Some(f64::from_ne_bytes(self.data[..8].try_into().unwrap()))
        } else {
            None
        }
    }

    fn as_bool(&self) -> Option<bool> {
        if self.typetag == DataType::Bool && !self.data.is_empty() {
            Some(self.data[0] != 0)
        } else {
            None
        }
    }
}

struct Database {
    db: HashMap<String, DbValue>,
}

impl Database {
    fn new() -> Self {
        Database {
            db: HashMap::new()
        }
    }

    fn get(&self, key: &str) -> Option<&DbValue> {
        self.db.get(key)
    }

    fn set(&mut self, key: String, value: DbValue) {
        self.db.insert(key, value);
    }
}

fn main() {
    let mut db = Database::new();
    
    loop {
        let mut input = String::new();  
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().split(" ").collect::<Vec<&str>>();

        if input.is_empty() {
            continue;
        }

        match input[0].to_uppercase().as_str() { // <== INPUT[0] = COMMAND
            "GET" => {
                if input.len() < 2 {
                    println!("Usage: GET <key>");
                    continue;
                }

                match db.get(input[1]) {
                    Some(value) => {
                        match value.typetag {
                            DataType::String => println!("{}: {}", input[1], value.as_string().unwrap()),
                            DataType::Int => println!("{}: {}", input[1], value.as_int().unwrap()),
                            DataType::Float => println!("{}: {}", input[1], value.as_float().unwrap()),
                            DataType::Bool => println!("{}: {}", input[1], value.as_bool().unwrap()),
                        }
                    },
                    None => println!("Key not found"),
                }
            },
            "SET" => {
                if input.len() < 3 {
                    println!("Usage: SET <key> <type> <value>");
                    println!("Types: str, int, float, bool");
                    continue;
                };
                
                let key = input[1].to_string();
                let value_type = input[2].to_lowercase();
                let value_str = input[3..].join(" ");

                let value = match value_type.as_str() {
                    "str" | "string" => DbValue::from_str(&value_str),
                    "int" | "i64" => match value_str.parse::<i64>() {
                        Ok(i) => DbValue::from_i64(i),
                        Err(_) => {
                            println!("Invalid integer value");
                            continue;
                        }
                    },
                    "float" | "f64" => match value_str.parse::<f64>() {
                        Ok(f) => DbValue::from_f64(f),
                        Err(_) => {
                            println!("Invalid float value");
                            continue;
                        }
                    },
                    "bool" => match value_str.parse::<bool>() {
                        Ok(b) => DbValue::from_bool(b),
                        Err(_) => {
                            println!("Invalid boolean value (use 'true' or 'false')");
                            continue;
                        }
                    },
                    _ => {
                        println!("Invalid type. Use: str, int, float, bool");
                        continue;
                    }
                };
                db.set(key, value);
                println!("SET successful");
            },
            "DEBUG" => {
                for (key, value) in &db.db {
                    println!("{}: {:?}", key, value);
                }
            },
            _ => {
                println!("Unknown command")
            }
        }
    }
}
