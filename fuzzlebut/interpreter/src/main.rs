use std::{
    fmt::{
        Write as WriteFmt,
        Display,
        Formatter,
        Result as FmtResult,
    },
    io::{
        stdin,
        stdout,
        Write,
    },
    env::args,
    fs::read_to_string,
    collections::HashMap,
};
use rand::{
    rngs::SmallRng,
    Rng,
    SeedableRng,
};
use parser::{
    Cells,
    Cell,
};
use stack::Stack;


type ProgramStack=Stack<Value>;


/// A generic value that can be used in programs
#[derive(Clone,Debug,PartialEq)]
enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Object(HashMap<String,Self>),
    List(Vec<Self>),
}
impl Display for Value {
    fn fmt(&self,f:&mut Formatter)->FmtResult {
        use Value::*;
        match self {
            String(s)=>f.write_str(&s),
            Int(i)=>write!(f,"{}",i),
            Float(i)=>write!(f,"{}",i),
            Bool(b)=>b.fmt(f),
            Char(c)=>c.fmt(f),
            List(l)=>{
                f.write_str("[")?;
                for (i,item) in l.iter().enumerate() {
                    match item {
                        String(s)=>{
                            f.write_str("\"")?;
                            f.write_str(s)?;
                            f.write_str("\"")?;
                        },
                        v=>v.fmt(f)?,
                    }
                    if i+1<l.len() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("]")
            },
            Object(o)=>{
                f.write_str("{")?;
                for (i,(k,v)) in o.iter().enumerate() {
                    f.write_str("\"")?;
                    f.write_str(k)?;
                    f.write_str("\"")?;
                    f.write_str(": ")?;
                    match v {
                        String(s)=>{
                            f.write_str("\"")?;
                            f.write_str(s)?;
                            f.write_str("\"")?;
                        },
                        v=>v.fmt(f)?,
                    }
                    if i+1<o.len() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("}")
            },
        }
    }
}
#[derive(Copy,Clone,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn turn_cw(&mut self) {
        use Direction::*;
        match self {
            Up=>*self=Right,
            Right=>*self=Down,
            Down=>*self=Left,
            Left=>*self=Up,
        }
    }
    fn turn_ccw(&mut self) {
        use Direction::*;
        match self {
            Up=>*self=Left,
            Left=>*self=Down,
            Down=>*self=Right,
            Right=>*self=Up,
        }
    }
}


#[derive(Copy,Clone,Debug)]
struct Rect {
    tl:[usize;2],
    br:[usize;2],
}
/// Holds vital information for a program to run
struct ProgramState {
    stacks:Stack<ProgramStack>,
    direction:Direction,
    cursors:Stack<[usize;2]>,
    cells:Cells,
    borders:Stack<Rect>,
    functions:HashMap<String,(Rect,usize)>,
    rng:SmallRng,
}
impl ProgramState {
    fn new(mut cells:Cells)->Self {
        let max=cells.iter().map(|row|row.len()).max().unwrap_or(0);
        for row in cells.iter_mut() {
            while row.len()<max {
                row.push(Cell::Nop);
            }
        }
        let mut stacks=Stack::new();
        stacks.push(ProgramStack::new());
        let mut cursors=Stack::new();
        cursors.push([0,0]);
        let mut borders=Stack::new();
        borders.push(Rect{tl:[0,0],br:[max-1,cells.len()-1]});
        ProgramState {
            borders,
            stacks,
            cursors,
            cells,
            direction:Direction::Right,
            functions:HashMap::new(),
            rng:SmallRng::from_entropy(),
        }
    }
    fn next_cell(&mut self) {
        use Direction::*;
        match self.direction {
            Up=>{
                if self.cursors[0][1]==self.borders[0].tl[1] {
                    panic!("Error: hit the up edge at {:?}",self.cursors[0]);
                }
            },
            Down=>{
                if self.cursors[0][1]==self.borders[0].br[1] {
                    panic!("Error: hit the down edge at {:?}",self.cursors[0]);
                }
            },
            Left=>{
                if self.cursors[0][0]==self.borders[0].tl[0] {
                    panic!("Error: hit the left edge at {:?}",self.cursors[0]);
                }
            },
            Right=>{
                if self.cursors[0][0]==self.borders[0].br[0] {
                    panic!("Error: hit the right edge at {:?}",self.cursors[0]);
                }
            },
        }
        match self.direction {
            Up=>self.cursors[0][1]-=1,
            Down=>self.cursors[0][1]+=1,
            Left=>self.cursors[0][0]-=1,
            Right=>self.cursors[0][0]+=1,
        }
    }
    fn prev_cell(&mut self) {
        use Direction::*;
        match self.direction {
            Up=>{
                if self.cursors[0][1]==self.borders[0].br[1] {
                    panic!("Error: hit an edge at {:?}",self.cursors[0]);
                }
            },
            Down=>{
                if self.cursors[0][1]==self.borders[0].tl[1] {
                    panic!("Error: hit an edge at {:?}",self.cursors[0]);
                }
            },
            Left=>{
                if self.cursors[0][0]==self.borders[0].br[0] {
                    panic!("Error: hit an edge at {:?}",self.cursors[0]);
                }
            },
            Right=>{
                if self.cursors[0][0]==self.borders[0].tl[0] {
                    panic!("Error: hit an edge at {:?}",self.cursors[0]);
                }
            },
        }
        match self.direction {
            Up=>self.cursors[0][1]+=1,
            Down=>self.cursors[0][1]-=1,
            Left=>self.cursors[0][0]+=1,
            Right=>self.cursors[0][0]-=1,
        }
    }
    fn current_cell(&self)->&Cell {
        &self.cells[self.cursors[0][1]][self.cursors[0][0]]
    }
    /// Evaluate the program
    fn run(&mut self) {
        use Cell::*;
        use Direction::*;
        loop {
            match self.current_cell() {
                DoubleQuote=>{
                    let mut string=String::new();
                    loop {
                        self.next_cell();
                        match self.current_cell() {
                            DoubleQuote=>break,
                            Other('\\')=>{
                                self.next_cell();
                                match self.current_cell() {
                                    DoubleQuote=>string.push('"'),
                                    Other('n')=>string.push('\n'),
                                    Other('r')=>string.push('\r'),
                                    Other('t')=>string.push('\t'),
                                    Other('\\')=>string.push('\\'),
                                    Number('0')=>string.push('\0'),
                                    c=>panic!("Invalid escape sequence `{}` at {:?}",c.into_char(),self.cursors[0]),
                                }
                            },
                            c=>string.push(c.into_char()),
                        }
                    }
                    self.stacks[0].push(Value::String(string));
                },
                Number(n)=>{
                    let mut number=String::from(*n);
                    let mut float=false;
                    loop {
                        self.next_cell();
                        match self.current_cell() {
                            Dot=>{
                                if float {
                                    panic!("There can only be one decimal for each float. Location: {:?}",self.cursors[0]);
                                }
                                float=true;
                                number.push('.');
                            },
                            Number(n)=>number.push(*n),
                            _=>{
                                self.prev_cell();
                                break;
                            },
                        }
                    }
                    if float {
                        self.stacks[0].push(Value::Float(number.parse().unwrap()));
                    } else {
                        self.stacks[0].push(Value::Int(number.parse().unwrap()));
                    }
                },
                Add=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    match (left,right) {
                        (Value::List(mut items),right)=>{
                            items.push(right);
                            self.stacks[0].push(Value::List(items));
                        },
                        (Value::String(mut s),right)=>{
                            write!(s,"{}",right).unwrap();
                            self.stacks[0].push(Value::String(s));
                        },
                        (Value::Int(mut i1),Value::Int(i2))=>{
                            i1+=i2;
                            self.stacks[0].push(Value::Int(i1));
                        },
                        (Value::Float(mut f1),Value::Float(f2))=>{
                            f1+=f2;
                            self.stacks[0].push(Value::Float(f1));
                        },
                        _=>panic!("Invalid types in add at {:?}",self.cursors[0]),
                    }
                },
                Sub=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    match (left,right) {
                        (Value::String(mut s),Value::Int(mut len))=>{
                            while s.len()>0&&len>0 {
                                s.pop();
                                len-=1;
                            }
                            self.stacks[0].push(Value::String(s));
                        },
                        (Value::Int(mut i1),Value::Int(i2))=>{
                            i1-=i2;
                            self.stacks[0].push(Value::Int(i1));
                        },
                        (Value::Float(mut f1),Value::Float(f2))=>{
                            f1-=f2;
                            self.stacks[0].push(Value::Float(f1));
                        },
                        (Value::List(mut items),Value::Int(mut len))=>{
                            while items.len()>0&&len>0 {
                                items.pop();
                                len-=1;
                            }
                            self.stacks[0].push(Value::List(items));
                        },
                        (Value::Object(mut fields),Value::String(s))=>{
                            fields.remove(&s);
                            self.stacks[0].push(Value::Object(fields));
                        },
                        _=>panic!("Invalid types in sub at {:?}",self.cursors[0]),
                    }
                },
                Mul=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    match (left,right) {
                        (Value::Int(mut i1),Value::Int(i2))=>{
                            i1*=i2;
                            self.stacks[0].push(Value::Int(i1));
                        },
                        (Value::Float(mut f1),Value::Float(f2))=>{
                            f1*=f2;
                            self.stacks[0].push(Value::Float(f1));
                        },
                        _=>panic!("Invalid types in mul at {:?}",self.cursors[0]),
                    }
                },
                Div=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    match (left,right) {
                        (Value::Int(i1),Value::Int(i2))=>{
                            let res=i1/i2;
                            let rem=i1%i2;
                            self.stacks[0].push(Value::Int(res));
                            self.stacks[0].push(Value::Int(rem));
                        },
                        (Value::Float(f1),Value::Float(f2))=>{
                            let res=f1/f2;
                            let rem=f1%f2;
                            self.stacks[0].push(Value::Float(res));
                            self.stacks[0].push(Value::Float(rem));
                        },
                        _=>panic!("Invalid types in div at {:?}",self.cursors[0]),
                    }
                },
                Object=>self.stacks[0].push(Value::Object(HashMap::new())),
                Field=>{
                    let object=self.stacks[0].pop().unwrap();
                    let name=self.stacks[0].pop().unwrap();
                    let data=self.stacks[0].pop().unwrap();
                    match (object,name) {
                        (Value::Object(mut o),Value::String(name))=>{
                            o.insert(name,data);
                            self.stacks[0].push(Value::Object(o));
                        },
                        _=>panic!("Invalid types for field create at {:?}",self.cursors[0]),
                    }
                },
                SetUp=>self.direction=Up,
                SetDown=>self.direction=Down,
                SetLeft=>self.direction=Left,
                SetRight=>self.direction=Right,
                WireVert=>{
                    match self.direction {
                        Left|Right=>panic!("Hit a vertical wire while going horizontal at {:?}. Please use direction changes.",self.cursors[0]),
                        _=>{},
                    }
                },
                WireHoriz=>{
                    match self.direction {
                        Up|Down=>panic!("Hit a horizontal wire while going vertical at {:?}. Please use direction changes.",self.cursors[0]),
                        _=>{},
                    }
                },
                WireCross=>{},
                Negate=>{
                    match &mut self.stacks[0][0] {
                        Value::Int(i)=>*i*=-1,
                        Value::Float(f)=>*f*=-1.0,
                        Value::Bool(b)=>*b=!*b,
                        _=>{},
                    }
                },
                Delete=>{self.stacks[0].pop();},
                NumberCast=>{
                    match self.stacks[0].pop().unwrap() {
                        Value::Float(f)=>self.stacks[0].push(Value::Int(f as i64)),
                        Value::Int(i)=>self.stacks[0].push(Value::Float(i as f64)),
                        i=>self.stacks[0].push(i),
                    }
                },
                Print=>{
                    print!("{}",self.stacks[0][0]);
                    stdout().flush().unwrap();
                },
                Println=>{
                    println!("{}",self.stacks[0][0]);
                },
                ProcDef=>{
                    let name=match self.stacks[0].pop().unwrap() {
                        Value::String(s)=>s,
                        _=>panic!("Invalid type for procedure arg count at {:?}. Expected String",self.cursors[0]),
                    };
                    let arg_count=match self.stacks[0].pop().unwrap() {
                        Value::Int(i)=>i,
                        _=>panic!("Invalid type for procedure arg count at {:?}. Expected Int",self.cursors[0]),
                    };
                    if arg_count<0 {
                        panic!("Procedure argument count should be above 0");
                    }
                    let tl=self.cursors[0];
                    let mut br=self.cursors[0];
                    self.direction=Right;
                    loop {
                        self.next_cell();
                        br[0]+=1;
                        match self.current_cell() {
                            WireVert=>{
                                self.direction=Down;
                                break;
                            },
                            _=>{},
                        }
                    }
                    loop {
                        self.next_cell();
                        br[1]+=1;
                        match self.current_cell() {
                            WireHoriz=>{
                                self.direction=Down;
                                break;
                            },
                            _=>{},
                        }
                    }
                    self.direction=Left;
                    for _ in tl[0]..br[0] {
                        if self.current_cell()!=&WireHoriz {
                            panic!("Invalid cell {:?} at {:?}. Expected WireHoriz",self.current_cell(),self.cursors[0]);
                        }
                        self.next_cell();
                    }
                    self.direction=Up;
                    for _ in tl[1]..(br[1]-1) {
                        if self.current_cell()!=&WireVert {
                            panic!("Invalid cell {:?} at {:?}. Expected WireVert",self.current_cell(),self.cursors[0]);
                        }
                        self.next_cell();
                    }
                    self.cursors[0]=br;
                    let rect=Rect{tl,br};
                    self.functions.insert(name,(rect,arg_count as usize));
                    self.direction=Right;
                },
                Greater=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    match (left,right) {
                        (Value::Float(f1),Value::Float(f2))=>self.stacks[0].push(Value::Bool(f1>f2)),
                        (Value::Int(i1),Value::Int(i2))=>self.stacks[0].push(Value::Bool(i1>i2)),
                        _=>self.stacks[0].push(Value::Bool(false)),
                    }
                },
                Less=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    match (left,right) {
                        (Value::Float(f1),Value::Float(f2))=>self.stacks[0].push(Value::Bool(f1<f2)),
                        (Value::Int(i1),Value::Int(i2))=>self.stacks[0].push(Value::Bool(i1<i2)),
                        _=>self.stacks[0].push(Value::Bool(false)),
                    }
                },
                Equality=>{
                    let left=self.stacks[0].pop().unwrap();
                    let right=self.stacks[0].pop().unwrap();
                    self.stacks[0].push(Value::Bool(left==right));
                },
                True=>self.stacks[0].push(Value::Bool(true)),
                Exit=>break,
                ListCreate=>self.stacks[0].push(Value::List(Vec::new())),
                Rotate=>self.stacks[0].rotate(),
                RotateRev=>self.stacks[0].rotate_rev(),
                RunProc=>{
                    let name=match self.stacks[0].pop().unwrap() {
                        Value::String(s)=>s,
                        _=>panic!("Invalid type for procedure arg count at {:?}. Expected String",self.cursors[0]),
                    };
                    if let Some((boundary,args))=self.functions.get(&name) {
                        self.borders.push(*boundary);
                        let mut cursor=boundary.tl.clone();
                        cursor[0]+=1;
                        cursor[1]+=1;
                        self.cursors.push(cursor);
                        self.stacks.push(ProgramStack::new());
                        let mut items=Vec::new();
                        for _ in 0..*args {
                            let arg=self.stacks[1].pop().unwrap();
                            items.push(arg);
                        }
                        for item in items.into_iter().rev() {
                            self.stacks[0].push(item);
                        }
                        self.run();
                        self.borders.pop();
                        if let Some(item)=self.stacks[0].pop() {
                            self.stacks[1].push(item);
                        }
                        self.cursors.pop();
                        self.stacks.pop();
                    } else {
                        panic!("Function {} not found at {:?}",name,self.cursors[0]);
                    }
                },
                UserInput=>{
                    let stdin=stdin();
                    let mut s=String::new();
                    stdin.read_line(&mut s).expect("Could not read STDIN");
                    while let Some(c)=s.pop() {
                        match c {
                            '\r'|'\n'=>{},  // support Windows, Unix, and MacOS linefeeds. Also just get rid of unwanted characters at the end of the user input.
                            _=>{
                                s.push(c);
                                break;
                            },
                        }
                    }
                    self.stacks[0].push(Value::String(s));
                },
                Swap=>{
                    let first=self.stacks[0].pop().unwrap();
                    let second=self.stacks[0].pop().unwrap();
                    self.stacks[0].push(first);
                    self.stacks[0].push(second);
                },
                Nop|Other(_)=>{},
                Duplicate=>{
                    let dup=self.stacks[0][0].clone();
                    self.stacks[0].push(dup);
                },
                Char=>{
                    self.next_cell();
                    let c=self.current_cell().into_char();
                    self.stacks[0].push(Value::Char(c));
                },
                Dot=>{  // String Split
                    match &self.stacks[0][0] {
                        Value::String(s)=>{
                            let val=Value::List(s.chars().map(Value::Char).collect());
                            self.stacks[0].push(val);
                        },
                        _=>{},
                    }
                },
                Pop=>{
                    match &mut self.stacks[0][0] {
                        Value::String(s)=>{
                            let c=s.pop().unwrap();
                            self.stacks[0].push(Value::Char(c));
                        },
                        Value::List(l)=>{
                            let item=l.pop().unwrap();
                            self.stacks[0].push(item);
                        },
                        _=>{},
                    }
                },
                Branch=>{
                    match self.stacks[0].pop().unwrap() {
                        Value::Bool(true)=>{
                            self.direction.turn_ccw();
                        },
                        _=>self.direction.turn_cw(),
                    }
                },
                BranchRev=>{
                    match self.stacks[0].pop().unwrap() {
                        Value::Bool(true)=>{
                            self.direction.turn_cw();
                        },
                        _=>self.direction.turn_ccw(),
                    }
                },
                Length=>{
                    match &self.stacks[0][0] {
                        Value::String(s)=>{
                            let len=s.chars().count();
                            self.stacks[0].push(Value::Int(len as i64));
                        },
                        Value::List(l)=>{
                            let len=l.len();
                            self.stacks[0].push(Value::Int(len as i64));
                        },
                        Value::Object(o)=>{
                            let len=o.len();
                            self.stacks[0].push(Value::Int(len as i64));
                        },
                        _=>self.stacks[0].push(Value::Int(0)),
                    }
                },
                Debug=>self.stacks[0].debug_print(),
                RandInt=>{
                    let max=self.stacks[0].pop().unwrap();
                    let min=self.stacks[0].pop().unwrap();
                    match (min,max) {
                        (Value::Int(0),Value::Int(0))=>{
                            let random=self.rng.gen();
                            self.stacks[0].push(Value::Int(random));
                        },
                        (Value::Int(min),Value::Int(max))=>{
                            let random=self.rng.gen_range(min..=max);
                            self.stacks[0].push(Value::Int(random));
                        },
                        (l,r)=>{
                            self.stacks[0].push(l);
                            self.stacks[0].push(r);
                            let random=self.rng.gen();
                            self.stacks[0].push(Value::Int(random));
                        },
                    }
                },
                RandFloat=>{
                    let max=self.stacks[0].pop().unwrap();
                    let min=self.stacks[0].pop().unwrap();
                    match (min,max) {
                        (Value::Float(min),Value::Float(max))=>{
                            if min==max&&min==0.0 {
                                let random=self.rng.gen();
                                self.stacks[0].push(Value::Float(random));
                            } else {
                                let random=self.rng.gen_range(min..=max);
                                self.stacks[0].push(Value::Float(random));
                            }
                        },
                        (l,r)=>{
                            self.stacks[0].push(l);
                            self.stacks[0].push(r);
                            let random=self.rng.gen();
                            self.stacks[0].push(Value::Float(random));
                        },
                    }
                },
            }
            self.next_cell();
        }
    }
}


fn main() {
    for filename in args().skip(1) {
        // Run every file passed to it
        let contents=read_to_string(filename).unwrap();
        let cells=parser::from_source(&contents);
        let mut program=ProgramState::new(cells);
        program.run();
    }
}
