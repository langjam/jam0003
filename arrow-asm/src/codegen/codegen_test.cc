#include "codegen.hh"

using namespace codegen;
using namespace bytecode;

int main() {
    BytecodeChunk chunk;
    chunk.code = {
        { kImmByte, 3 },
        { kImmByte, 5 },
        { kAddLong, 0 },
        { kImmByte, 2 },
        { kMulLong, 0 },
        { kPrintLong, 0 },
        { kReturn, 0 }
    };
    BytecodeExecutable exec;
    exec.chunks = {chunk};
    exec.chunk_locations = {0};
    
    Generate(exec, "test.o");
}
