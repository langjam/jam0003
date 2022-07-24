#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Value {
    pub fn type_id(&self) -> u8 {
        match self {
            Value::Number(_) => 0,
            Value::String(_) => 1,
            Value::Bool(_) => 2,
        }
    }

    pub fn truthy(&self) -> bool {
        match self {
            Value::Number(number) => *number > 0.0,
            Value::String(string) => string.len() > 0,
            Value::Bool(boolean) => *boolean,
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(number) => number.to_string(),
            Value::String(string) => string.clone(),
            Value::Bool(boolean) => {
                if *boolean {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
        }
    }
}
