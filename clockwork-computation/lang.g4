grammar lang;

program: component_declaration+;

component_declaration: 'component' component_name '{' component_body '}';
component_name: IDENTIFIER;
component_body: part_or_use*;

part_or_use: part | use;
part: designator? part_name ':' part_type '{' part_body '}';
part_name: IDENTIFIER;
designator: 'input' | 'output';
part_type: 'gear' | 'rod';
part_body: part_body_item*;

part_body_item: option | connection;

option: option_name ':' option_value;
option_name: IDENTIFIER;
option_value: IDENTIFIER | NUMBER;

connection: part_name '->' part_name '{' connection_options '}';
connection_options: option*;

use: 'use' component_name '(' parameters ')' '->' outputs;
parameters: (part_name (',' part_name)* )?;
outputs: (part_name (',' part_name)* );



NUMBER: [0-9]+;
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
WS : [ \t\r\n]+ -> skip;
COMMENT: '//' (~'\n')* -> skip;