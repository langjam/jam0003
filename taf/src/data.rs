#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub enum Tok {
    Point(String),  // @define a function (but a point can also be a pointer into the program)
  //Star,           // *run to the next end, adding all Toks to the program stack without
                    //  excecuting them. unimplemented for langjam
    End,            // ~pop the program stack and return to the parent function
    Ptr(String),    // #a pointer into the point
    Call,           // !pop & call

    String(String),
    Int(u64)
}

#[derive(Debug)]
pub enum Point {
    User(usize),
    Print,
    Add,
    Dec,
    Dupn,
    Spot,
    Gt,             // >temporary       pc=x>y?pc+1:pc
}
