from sys import argv

def valid_syntax(x):
  if valid_digit(x) or valid_string(x): return True
  else: return False

def valid_digit(x):
  if x is None: return False
  return bool(sum([i in digits for i in str(x)]))

def valid_string(x):
  if x is None: return False
  is_quote = False
  for i in x:
    if i == "\"": is_quote = not is_quote
    else:
      if not is_quote: return False
  if is_quote: return False
  else: return True

def err_msg(msg):
  print(f"  in Line {line+1}: {program[line]} --> {msg}")
  print("Exiting...")
  exit(1)
  
if len(argv) < 2:
  print(f"Usage: {argv[0]} FILE")
  exit(1)

try:
  program = open(argv[1],"r").read().split('\n')
except:
  print(f"Cannot open '{argv[1]}'")
  exit(1)
  
# program = ['//' if len(i)==0 else i for i in program]
op,d0,d1,d2,d3 = None,None,None,None,None
memory = [None] * 64
digits = "0123456789"
line = 0

while line < len(program):
  token = program[line].split()
  if len(token) == 0: line+=1; continue
  comm = token[0]
  args = ' '.join(token[1:]) if len(token) > 1 else None
  
  if comm == "//": pass
  elif comm in ['op','d0','d1','d2','d3']:
    if valid_syntax(args): exec(f"{comm} = {args}")
    else: err_msg("invalid argument")
  elif comm == "call":
    if op == 0:
      if valid_digit(d0): memory[d0] = d1
      else: err_msg("invalid syntax")
    elif op == 1:
      if valid_digit(d0):
        print(memory[d0],end='')
    elif op == 2:
      if not valid_digit(d0): err_msg("invalid 'from' index")
      if not valid_digit(d1): err_msg("invalid 'to' index")
      memory[d1] = memory[d0]
      memory[d0] = None
    elif op == 3:
      if valid_digit(d0) and valid_digit(d1) and valid_digit(d2):
        try: memory[d2] = memory[d0] + memory[d1]
        except: err_msg("invalid type for PLUS (+) operation")
      else: err_msg("invalid syntax")
    elif op == 4:
      if valid_digit(d0) and valid_digit(d1) and valid_digit(d2):
        try: memory[d2] = memory[d0] - memory[d1]
        except: err_msg("invalid type for MINUS (-) operation")
      else: err_msg("invalid syntax")
    elif op == 5:
      if valid_digit(d0) and valid_digit(d1) and valid_digit(d2):
        try: memory[d2] = memory[d0] * memory[d1]
        except: err_msg("invalid type for MULTI (*) operation")
      else: err_msg("invalid syntax")
    elif op == 6:
      if valid_digit(d0) and valid_digit(d1) and valid_digit(d2):
        try: memory[d2] = memory[d0] / memory[d1]
        except: err_msg("invalid type for DIV (/) operation")
      else: err_msg("invalid syntax")
    elif op == 7:
      if valid_digit(d0) and valid_digit(d1) and (valid_digit(d2) and d2 < len(program)):
        if memory[d0] == memory[d1]: 
          line = d2-1
          continue
      else: err_msg("invalid syntax")
    elif op == 8:
      if valid_digit(d0):
        line = d0-1
        continue
      else: err_msg("expected number for 'jump' on d0")
    elif op == 9:
      if valid_digit(d0):
        exit(d0)
      else: err_msg("expected exit-code on d0")
    else: print(op);err_msg("unexpected operation number")
    
    op,d0,d1,d2,d3 = None,None,None,None,None
  else:
    err_msg(f"unknown command '{comm}'")
    exit(1)
  line += 1
