from pyparsing import *

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

record_prpty = Group(name + Suppress("=") + val_expr[...], aslist=True)
record_val = Suppress("{") + eol[...] + Opt(
    record_prpty + (("," | eol) + eol[...] + record_prpty)[...]
  ) + eol[...] + Suppress("}")
bag_val = Suppress("[") + (val_expr | eol)[...] + Suppress("]")

type_expr <<= Word(alphas)[...]
val_expr <<= term | record_val | bag_val

data = LineStart() + CaselessKeyword("data") + term + ":" + Group(type_expr[...], aslist=True) + eol
let = LineStart() + CaselessKeyword("let") + term + "=" + Group(val_expr[...], aslist=True) + eol

whole_file = (data | let | eol)[...] + val_expr[...] + eol[...]

def parse_file(file):
  output = whole_file.parse_file(file, parse_all=True)
  return (DATA_items, LET_items, output)

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
