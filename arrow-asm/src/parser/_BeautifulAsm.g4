grammar BeautifulAsm;

program : (statement)*;

RREG  : 'rr'[0-9]+;
LREG  : 'l'[0-9]+;
IPREG : 'p'[0-9]+;
OPREG : 'op'[0-9]+;
SREG  : 'sr';

ID  : [_a-z]+[_a-z0-9]*;

INT_NUM : [-]?('0' | [1-9][0-9]*);
FLOAT_NUM : INT_NUM '.' ([0-9]*[1-9])?;

WS  : [ \t\r\n]+ -> skip;
COMMENT : [;][^\r\n]* -> skip;

any_number : INT_NUM | FLOAT_NUM;

ARROW : '<-';

FN   : 'fn';
TYPE : 'type';

statement : function_definition
          | type_definition;

any_lvalue : RREG | LREG | IPREG | OPREG;
any_rvalue : LREG | IPREG | OPREG | SREG;

LONG   : 'long';
DOUBLE : 'double';
PTR    : 'ptr';

register_type : LONG
              | DOUBLE 
              | PTR '<' datatype=object_type '>';

object_type : ID
            | register_type;

type_definition : TYPE name=ID '{' (constructor | destructor | field)* '}';

CTOR : 'ctor';
DTOR : 'dtor';
constructor : CTOR '{' function_body '}';
destructor  : DTOR '{' function_body '}';

field : field_name=ID ':' field_type=register_type;

parameter_list : '(' type=register_type (',' type=register_type)* ')';
function_definition : FN name=ID parameter_list? '{' instruction* '}';

instruction : arrow_instruction
            | no_arg_instruction
            | binary_operator_instruction
            | memory_instruction
            | if_statement;

no_arg_instruction          : operator=no_arg_operator;
arrow_instruction           : lhs=arrow_lhs ARROW rhs=arrow_rhs;
print_instruction           : 'print' arg1=any_argument;
binary_operator_instruction : operator=binary_operator arg1=any_lvalue ',' arg2=any_argument ',' arg3=any_argument;
memory_instruction          : operator=memory_operator arg1=any_lvalue ',' arg2=memory_destination;

IF : 'if';
ELSE : 'else';
if_statement : IF condition=any_argument '{' instruction* '}' elif_branch* else_branch?;
elif_branch  : ELSE condition=any_argument '{' instruction* '}';
else_branch  : ELSE '{' instruction* '}';

any_argument : any_rvalue | any_number;

arrow_lhs : any_lvalue
          | any_field;

arrow_rhs : make_constructor
          | any_rvalue
          | any_field;

MAKE : 'make';
make_constructor : MAKE type=ID (any_argument)*;

any_field : any_rvalue '.' field_name=ID;

memory_destination : any_rvalue
                   | any_field;

no_arg_operator : 'nop'
                | 'trap'
                | 'ret'
                | 'break'
                | 'continue';

binary_operator : 'add'
                | 'mul';

memory_operator : 'load'
                | 'store';