# runner.
from tallyAst import *

def run_program(program, json):
  print(program) #for debug
  print(json) #also debug
  
  global lets
  lets = program.lets
  
  for expr in program.exprs:
    json = apply_expr(expr, json, {})
  
  print("Done!")
  return json

def apply_expr(expr, json, env):
  print("-- applying ", expr, " to ", json)
  ## this might not be supre idiomatic python,
  ## to have 'thin' objects and give them behavious elsewhere,
  ## but I don't usually write python and prefer it this way.
  match expr:
    case Term(name, params):
      print("----TERM")
      if name == "BUILTIN": return apply_builtin(params[0], json)
      if name not in lets: raise Exception("Use of non-existent term '" + name + "'")
      j = json
      for e in lets[name].exprs:
        j = apply_expr(e, j, env) # TODO env should be adjusted for lets[name].params
      return j

def apply_builtin(which, json):
  print("-- applying BUILTIN ", which, " to ", json)
  match which:
    case "identity": return json