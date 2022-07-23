#include "parser.h"
#include "common.h"
#include "vmintrin.h"
#include "debug.h"

#define UNHANG {static int c = 0; if (c++ > 1000) {CHECKOUT(1);}}
#define CHECKOUT(x) if (x) { tprintf("LAST SEEN: {}\n", __LINE__); return 1; }
#define CPINSTR_MAX (1 << 12)
static Instr current_prog_instrs[CPINSTR_MAX];
static Program current_prog;

struct Parser {
  const char *source;
  bool going;
  uint line, col;
};

enum TokenType {
  TokenType_Null,
  TokenType_Immediate, // number
  TokenType_Register, 
  TokenType_Instr, 
};

struct Token {
  TokenType type;
  const char *str;
  uint len, line, col;
};

u8 parser_get(Parser *p) {
  if (*p->source == '\0') {
    p->going = false;
    return 0;
  }
  return *p->source;
}

bool parser_is(Parser *p, u8 against) {
  return *p->source == against && *p->source != 0;
}

bool parser_isnt(Parser *p, u8 against) {
  return *p->source != against && *p->source != 0;
}

u8 parser_next(Parser *p) {
  u8 old = *p->source;
  if (old == '\0') {
    p->going = false;
    return 0;
  }

  if (old == '\n') {
    p->line += 1;
    p->col = 0;
  } else {
    p->col += 1;
  }
  p->source++;
  return old;
}

static bool skip_empty(Parser *p) {
  if (parser_is(p, ' ') || parser_is(p, '\r') || parser_is(p, '\n') || parser_is(p, '\t')) {
    parser_next(p);
    return true;
  } else if (parser_is(p, ';')) {
    while (parser_isnt(p, '\n')) {
      parser_next(p);
    }
    return true;
  }

  return false;
}

static bool isalpha(u8 c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

static bool isnum(u8 c) {
  return c >= '0' && c <= '9';
}

static bool isalnum(u8 c) {
  return isnum(c) || isalpha(c);
}

static bool fetch_token(Parser *p, Token *output) {
  while (skip_empty(p)) 
    ;



  Token res = {};
  res.line = p->line;
  res.col = p->col;
  res.str = p->source;

  if (parser_is(p, '%')) {
    res.type = TokenType_Register;
    parser_next(p);
    int l = 0;
    while (isalnum(parser_get(p))) {
      parser_next(p);
      l++;
    }
    if (l <= 0) {
      // TODO: Log error
      return true;
    }
  } else if (isnum(parser_get(p))) {
    res.type = TokenType_Immediate;
    while (isnum(parser_get(p))) {
      parser_next(p);
    }
  } else if (isalpha(parser_get(p))) {
    res.type = TokenType_Instr;
    while (isalnum(parser_get(p))) {
      parser_next(p);
    }
  } else if (p->going == false) {

  } else {
    // TODO: Log error
    return true;
  }

  res.len = p->source-res.str;
  *output = res;

  return false;
}

bool parser_put_instr(Parser *p, Instr instr) {
  (void)p;
  if (current_prog.instrs_len >= CPINSTR_MAX) {
    // TODO: Log error
    return true;
  }
  current_prog.instrs[current_prog.instrs_len++] = instr;
  return false;
}

float token_to_number(Token t) {
  float res = 0;
  while (isnum(*t.str)) {
    res = res * 10 + (*t.str++ - '0');
  }

  return res;
}

bool token_to_register(Token t, Reg *reg_out) {
  if (t.type != TokenType_Register) {
    // TODO: Log error;
    CHECKOUT(1);
  }

  Span registers[REG_COUNT];
  registers[Reg_X] = Span{"X", 1};
  registers[Reg_Y] = Span{"Y", 1};
  registers[Reg_Ret] = Span{"RET", 3};
  registers[Reg_Out] = Span{"OUT", 3};
  t.str += 1; // skip %
  t.len -= 1; //
  for (uint i = 0; i < REG_COUNT; ++i) {
    if (span_equal(registers[i], Span{t.str, t.len})) {
      *reg_out = (Reg)i;
      return 0;
    }
  }
  tprintf("Unknown register: {}\n", Span{t.str, t.len});
  // TODO: Log error
  CHECKOUT(1);
}

bool emit_value(Parser *p, Token t) {
  switch (t.type) {
    case TokenType_Immediate: {
      float fv = token_to_number(t);
      CHECKOUT(parser_put_instr(p, Instr{InstrType_Imm, *reinterpret_cast<uint*>(&fv)}));
    } break;
    case TokenType_Register: {
      Reg reg;
      CHECKOUT(token_to_register(t, &reg));
      CHECKOUT(parser_put_instr(p, Instr{InstrType_Load, reg}));
    } break;
    default: {
      // TODO: Log error
      CHECKOUT(1);
    } break;
  }

   return 0;
}

bool parse_call(Parser *p, Token name) {
  if (name.type != TokenType_Instr) {
    // TODO: Log error
    tprintf("Error: `{}` Is not an instruction\n", Span{name.str, name.len});
    CHECKOUT(1);
  }


  if (span_equal({name.str, name.len}, {"INTO", 4})) {
    Token t;
    CHECKOUT(fetch_token(p, &t));
    Reg reg;
    CHECKOUT(token_to_register(t, &reg));
    CHECKOUT(parser_put_instr(p, Instr{InstrType_Store, reg}));
    CHECKOUT(fetch_token(p, &name));
  } else {
    // TODO: Handle too many/few params
again:
    Token t;
    CHECKOUT(fetch_token(p, &t));

    if (t.type == TokenType_Immediate || t.type == TokenType_Register) {
      CHECKOUT(emit_value(p, t));
      goto again;
    }

    if (span_equal({name.str, name.len}, {"ADD", 3})) {
      CHECKOUT(parser_put_instr(p, Instr{InstrType_Add}));
    } else if (span_equal({name.str, name.len}, {"MOD", 3})) {
      CHECKOUT(parser_put_instr(p, Instr{InstrType_Mod}));
    } else if (span_equal({name.str, name.len}, {"MUL", 3})) {
      CHECKOUT(parser_put_instr(p, Instr{InstrType_Mul}));
    } else {
      // TODO: Log error
      CHECKOUT(1);
    }

    name = t;
  }

  if (p->going) {
    CHECKOUT(parse_call(p, name));
  }
  return 0;
}

bool parse_tape(Parser *p) {
  Token t;
  CHECKOUT(fetch_token(p, &t));
  if (t.type == TokenType_Instr || t.type == TokenType_Register) {
    CHECKOUT(emit_value(p, t));
    CHECKOUT(fetch_token(p, &t));
  }

  CHECKOUT(parse_call(p, t));
  return 0;
}

bool parser_parse(Program *output, const char *source) {
  current_prog.instrs = current_prog_instrs;

  Parser p;
  p.source = source;
  p.going = true;
  while (p.going) {
    CHECKOUT(parse_tape(&p));
  }
  *output = current_prog;
  return false;
}
