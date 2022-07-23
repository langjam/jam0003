pub type Cells=Vec<Vec<Cell>>;


/// Cells are individual characters, but with semantic meaning.
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Cell {
    Nop,
    DoubleQuote,
    Add,
    Sub,
    Mul,
    Div,
    Object,
    Field,
    SetUp,
    SetDown,
    SetLeft,
    SetRight,
    WireVert,
    WireHoriz,
    WireCross,
    Negate,
    Delete,
    NumberCast,
    Print,
    Println,
    ProcDef,
    Greater,
    Less,
    Equality,
    True,
    Exit,
    ListCreate,
    Rotate,
    RotateRev,
    RunProc,
    UserInput,
    Swap,
    Duplicate,
    Char,
    Dot,
    Pop,
    Branch,
    BranchRev,
    Length,
    Debug,
    RandInt,
    RandFloat,
    Number(char),
    Other(char),
}
impl Cell {
    pub fn into_char(self)->char {
        use Cell::*;
        match self {
            Nop=>' ',
            DoubleQuote=>'"',
            Add=>'A',
            Sub=>'S',
            Mul=>'M',
            Div=>'D',
            Object=>'O',
            Field=>'F',
            SetUp=>'^',
            SetDown=>'v',
            SetLeft=>'<',
            SetRight=>'>',
            WireVert=>'|',
            WireHoriz=>'-',
            WireCross=>'+',
            Negate=>'N',
            Delete=>'%',
            NumberCast=>'C',
            Print=>'!',
            Println=>'#',
            ProcDef=>'P',
            Greater=>'G',
            Less=>'L',
            Equality=>'E',
            True=>'T',
            ListCreate=>'V',
            Exit=>'~',
            Rotate=>'R',
            RotateRev=>'r',
            RunProc=>'*',
            UserInput=>'U',
            Swap=>'s',
            Duplicate=>'d',
            Char=>'c',
            Dot=>'.',
            Pop=>'p',
            Branch=>'B',
            BranchRev=>'b',
            Length=>'l',
            Debug=>'?',
            RandInt=>'@',
            RandFloat=>'&',
            Number(n)=>n,
            Other(c)=>c,
        }
    }
    pub fn new(c:char)->Self {
        use Cell::*;
        match c {
            ' '=>Nop,
            '"'=>DoubleQuote,
            'A'=>Add,
            'S'=>Sub,
            'M'=>Mul,
            'D'=>Div,
            'O'=>Object,
            'F'=>Field,
            '^'=>SetUp,
            'v'=>SetDown,
            '<'=>SetLeft,
            '>'=>SetRight,
            '|'=>WireVert,
            '-'=>WireHoriz,
            '+'=>WireCross,
            'N'=>Negate,
            '%'=>Delete,
            'C'=>NumberCast,
            '!'=>Print,
            '#'=>Println,
            'P'=>ProcDef,
            'G'=>Greater,
            'L'=>Less,
            'E'=>Equality,
            'T'=>True,
            'V'=>ListCreate,
            '~'=>Exit,
            'R'=>Rotate,
            'r'=>RotateRev,
            '*'=>RunProc,
            'U'=>UserInput,
            's'=>Swap,
            'd'=>Duplicate,
            'c'=>Char,
            '.'=>Dot,
            'p'=>Pop,
            'B'=>Branch,
            'b'=>BranchRev,
            'l'=>Length,
            '?'=>Debug,
            '@'=>RandInt,
            '&'=>RandFloat,
            '0'..='9'=>Number(c),
            _=>Other(c),
        }
    }
}


/// Parse a file and create a grid of cells
pub fn from_source(source:&str)->Cells {
    source.lines().map(|line|{
        line.chars().map(Cell::new).collect()
    }).collect()
}