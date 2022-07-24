#pragma once

#include "src/ast/ast.hh"
#include "src/bytecode/bytecode.hh"
#include "src/bytecode/opcodes.hh"

namespace ast {

// TODO I think this should actually return a vector of chunks, and then we
// could have a pseudo-linking step connect those together (????maybe????)
bytecode::BytecodeExecutable LowerAst(const ProgramNode& ast);

} // namespace ast;
