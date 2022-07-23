#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
}

impl Value {
    // fn num(self, span: Span) -> Result<f64>
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

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value),
    Operator(Operator),
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
