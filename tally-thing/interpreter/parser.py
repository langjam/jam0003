from pyparsing import *
from tallyAst import *

DATA_items = {}
LET_items = {}

# Handle newlines explicitly
ParserElement.set_default_whitespace_chars(' \t')

type_expr = Forward()
val_expr = Forward()

comment = Suppress(Literal(";;") + rest_of_line)
eol = Suppress(Opt(comment) + "\n")

name = Word(alphas+"_'?", alphas+nums+"-#_<>?")
term = name + Opt(
    Suppress("(").leave_whitespace() + delimited_list(Word(alphas)) + Suppress(")")
  )

projection = Combine(Suppress(".") + Word(alphas))

record_prpty = Group(name + Suppress("=") + val_expr[...], aslist=True)
record_val = Suppress("{") + eol[...] + Opt(
    record_prpty + (("," | eol) + eol[...] + record_prpty)[...]
  ) + eol[...] + Suppress("}")
bag_val = Suppress("[") + (val_expr | eol)[...] + Suppress("]")

type_expr <<= Word(alphas)[...]
val_expr <<= term | projection | record_val | bag_val

data = LineStart() + CaselessKeyword("data") + term + ":" + Group(type_expr[...], aslist=True) + eol
let = LineStart() + CaselessKeyword("let") + term + "=" + Group(val_expr[...], aslist=True) + eol

whole_file = (data | let | eol)[...] + val_expr[...] + eol[...]

@term.set_parse_action
def create_term(tokens):
  if len(tokens) > 1:
    return Term(tokens[0], tokens[1:])
  else:
    return Term(tokens[0], [])

@projection.set_parse_action
def create_projection(tokens):
  return Projection(tokens[0])

@record_val.set_parse_action
def create_record_val(tokens):
  d = {}
  for prpty in tokens:
    d[prpty[0]] = prpty[1:]
  return RecordVal(d)
@bag_val.set_parse_action
def create_bag_val(tokens):
  return BagVal(tokens)

@data.set_parse_action
def register_data(tokens):
  global DATA_items
  DATA_items[tokens[1].name] = (tokens[1].params, tokens[3])
  return []
@let.set_parse_action
def register_let(tokens):
  global LET_items
  LET_items[tokens[1].name] = LetBody(tokens[1].params,tokens[3])
  return []

def parse_file(file):
  output = whole_file.parse_file(file, parse_all=True)
  return Program(DATA_items, LET_items, output)

if __name__ == "__main__":
  comment.run_tests("""\
    # comment
    ;; this is a comment
    """)
  let.run_tests("""\
    # let statement
    let greet = hello
    
    # let statement with compound expr
    let degreet = goodbye to us
    """)
  print(LET_items)
