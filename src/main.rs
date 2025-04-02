use std::{collections::HashMap, io::{self, Take}, str, u8};

#[derive(PartialEq)]
enum DataType {
    String  = 0,
    Int     = 1,
    Float   = 2,
    Bool    = 3,
}

struct DbValue {
    typetag: DataType,
    data: Vec<u8>,
}

// == To set data types easily. ==
impl DbValue {
    fn from_str(&self, s: &str) -> Self {
        let data = s.as_bytes().to_vec();
        DbValue {
            typetag: DataType::String,
            data,
        }
    }

    fn from_i32(&self, i: i32) -> Self {
        let data = i.to_string().as_bytes().to_vec();
        DbValue {
            typetag: DataType::Int,
            data,
        }
    }

    fn from_f32(&self, f: f32) -> Self {
        let data = f.to_string().as_bytes().to_vec();
        DbValue {
            typetag: DataType::Float,
            data,
        }
    }

    fn from_bool(&self, b: bool) -> Self {
        let data = if b { "true" } else { "false" }.as_bytes().to_vec();
        DbValue {
            typetag: DataType::Bool,
            data,
        }
    }
}

// == To get data types easily. ==
impl DbValue {
    fn as_string(&self) -> Option<&str> {
        if self.typetag == DataType::String {
            std::str::from_utf8(&self.data).ok()
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

        match input[0] { // <== INPUT[0] = COMMAND
            "GET" | "get" => {
                if input.len() < 2 {
                    println!("Usage: GET <key>");
                    continue;
                }

                let var = match db.get(input[1]) {
                    Some(var) => var,
                    None => {
                        println!("Key not found");
                        continue;
                    },
                };
                let var = match str::from_utf8(&var.value) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            
                println!("GET:TYPE {}:{} => {}", input[1], var);
            },
            "SET" | "set" => {
                if input.len() < 3 {
                    println!("Usage: SET <key:type> <value>");
                    continue;
                }
                let key =  match input[1].to_string();
                let vartype = match key.split(":").nth(1) {
                    Some(t) => t,
                    None => {
                        println!("Usage: SET <key:type> <value>");
                        continue;
                    }
                };
                let value = input[2].to_string().to_lowercase();
                let hash_value = match vartype {
                    "str" | "string" => HashValue { vartype: 0, value: value.into_bytes() },
                    "i32" | "int" => HashValue { vartype: 1, value: value.into_bytes() },
                    "f32" | "float" => HashValue { vartype: 2, value: value.into_bytes() },
                    _ => {
                        println!("Invalid type");
                        continue;
                    }
                };
                db.set(key);
                println!("SET {} => {}", input[1], value);
            },
            _ => {
                println!("Unknown command")
            }
        }
    }
}
