# runner.
from tallyAst import *

def run_program(program, json):
  print(program) #for debug
  print(json) #also debug
  
  for expr in program.exprs:
    apply_expr(expr, json, {})
  
  return "Done!"

def apply_expr(expr, json, env):
  print("-- applying ", expr, " to ", json)
  ## this might not be supre idiomatic python,
  ## to have 'thin' objects and give them behavious elsewhere,
  ## but I don't usually write python and prefer it this way.
  match expr:
    case Term(name, params):
      print("It's a term!")