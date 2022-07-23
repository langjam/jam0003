use std::fmt::Display;

pub type Ast = Vec<Expression>;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Component {
        id: String,
        fields: Vec<Expression>,
    },
    Pipe {
        id: String,
        ty: Box<Expression>,
    },
    Machine {
        id: String,
        params: Vec<Expression>,
        output: Option<Box<Expression>>,
        body: Vec<Expression>,
    },
    Factory {
        id: String,
        assembler: Vec<Expression>,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Number(value) => value.to_string(),
            Self::String(value) => value.to_owned(),
            Self::Boolean(value) => value.to_string(),
            Self::List(value) => format!("[{}]", value.iter().fold("".to_owned(), |acc, value| format!("{acc}, {}",value.to_string() ) )),
            Self::Component { id, fields } => format!("Component {{\n  id: {id}\n  fields: {fields:#?}\n}}"),
            Self::Pipe { id, ty } => format!("Pipe {{\n  id: {id}\n  ty: {ty:#?}\n}}"),
            Self::Machine { id, params, output, body } => format!("Machine {{\n  id: {id}\n  params: {params:#?}\n  output: {output:#?}\n body: {body:#?}\n}}"),
            Self::Factory { id, assembler } => format!("Factory {{\n  id: {id}\n  assembler: {assembler:#?}\n}}")
        };

        write!(f, "{message}")
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Negative(Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Modulus(Box<Expression>, Box<Expression>),

    Equal(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Negative(value) => format!("Negative {{ {value:#?} }}"),
            Self::Add(lhs, rhs) => format!("Add {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}"),
            Self::Subtract(lhs, rhs) => {
                format!("Subtract {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}")
            }
            Self::Multiply(lhs, rhs) => {
                format!("Multiply {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}")
            }
            Self::Divide(lhs, rhs) => format!("Divide {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}"),
            Self::Modulus(lhs, rhs) => format!("Modulus {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}"),

            Self::Equal(lhs, rhs) => format!("Equal {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}"),
            Self::GreaterThan(lhs, rhs) => {
                format!("GreaterThan {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}")
            }
            Self::LessThan(lhs, rhs) => {
                format!("LessThan LessThan{{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}")
            }
            Self::GreaterThanOrEqual(lhs, rhs) => {
                format!("GreaterThanOrEqual {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}")
            }
            Self::LessThanOrEqual(lhs, rhs) => {
                format!("LessThanOrEqual {{\n  lhs: {lhs:#?}\n  rhs: {rhs:#?}\n}}")
            }
            Self::Not(value) => format!("Not {{ {value:#?} }}"),
        };

        write!(f, "{message}")
    }
}

#[derive(Debug, Clone)]
pub enum Control {
    If {
        condition: Box<Expression>,
        block: Box<Expression>,
    },
    Else(Box<Expression>),
}

impl Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::If { condition, block } => {
                format!("If {{\n  condition: {condition:#?}\n  block: {block:#?}\n}}")
            }
            Self::Else(block) => format!("Else(block: {block:#?})"),
        };

        write!(f, "{message}")
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value),
    Operator(Operator),
    Control(Control),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Value(value) => format!("Value {{ {value} }}"),
            Self::Operator(operator) => format!("Operator {{ {operator} }}"),
            Self::Control(control) => format!("Control {{ {control} }}"),
        };

        write!(f, "{message}")
    }
}
