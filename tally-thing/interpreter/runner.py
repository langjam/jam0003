# runner.
from tallyAst import *
from itertools import groupby

def run_program(program, json):
  #print(program) #for debug
  #print(json) #also debug
  #print("\n---\n") #this too
  
  global lets
  lets = program.lets
  
  for expr in program.exprs:
    json = apply_expr(expr, json, {})
  
  return json

def apply_expr(expr, json, env):
  #print("-- applying ", expr )# , " to ", json)
  ## this might not be supre idiomatic python,
  ## to have 'thin' objects and give them behavious elsewhere,
  ## but I don't usually write python and prefer it this way.
  match expr:
    case Term(name, params):
      pars = list(env[p] if p in env else p for p in params)
      if name == "BUILTIN": return apply_builtin(pars, json)
      if name not in lets: raise Exception("Use of non-existent term '" + name + "'")
      j = json
      d = env
      for p, q in zip(lets[name].params, pars):
        d[p] = q
      for e in lets[name].exprs:
        j = apply_expr(e, j, d)
      return j
    case Projection(key):
      if not isinstance(json, dict):
        raise Exception("Tried to project using '." + key + "' into non-record data: " + str(json))
      if key not in json:
        raise Exception("Tried to access a member ('." + key + "') the record doesn't have: " + str(json))
      return json[key]

def apply_builtin(which, json):
  match which[0]:
    case "_identity": return json
    case "_by": 
      json = sorted(json, key=lambda x: x[which[1]])
      groups = []
      for k, g in groupby(json, lambda x: x[which[1]]):
        groups.append(list(g))      # Store group iterator as a list
      return groups
    case unknown: raise Exception("No such BUILTIN exists: " + unknown)