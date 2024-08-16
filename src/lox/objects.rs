#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Boolean(bool),
    String(String),
    Number(f32),
    Nil,
}

impl Object {
    pub fn to_string(self) -> String {
        match self {
            Object::Number(number) => number.to_string(),
            Object::String(string) => string,
            Object::Boolean(bool) => if bool {
                String::from("true")
            } else {
                String::from("false")
            },
            Object::Nil => String::from("nil"),
        }
    }
    pub fn bool(self) -> bool {
        if let Object::Boolean(bool) = self {
            bool
        } else {
            // Unreachable
            panic!("Not a bool!");
        }
    }

    pub fn is_string(&self) -> bool {
        if let Object::String(s) = self {
            return true
        } else {
            return false
        }
    }

    pub fn string(self) -> String {
        if let Object::String(string) = self {
            string
        } else {
            // Unreachable
            panic!("Not a string!");
        }
    }
    
    pub fn number(self) -> f32 {
        if let Object::Number(number) = self {
            number
        } else {
            // Unreachable
            panic!("Not a number!");
        }
    }

    pub fn is_number(&self) -> bool {
        if let Object::Number(s) = self {
            return true
        } else {
            return false
        }
    }
}
