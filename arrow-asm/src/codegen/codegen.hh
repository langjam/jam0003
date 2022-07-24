#pragma once
#include <string_view>

#include "src/bytecode/bytecode.hh"

namespace codegen {

// TODO: bytecode param
void Generate(bytecode::BytecodeExecutable& bytecode, std::string_view outputPath);

} // end namespace codegen
