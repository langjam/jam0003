#pragma once

#include <ast/exprs/expr.h>
#include <ast/instructions/instruction.h>
#include <runtime/state.h>

#include <vector>

class Interpreter {
   public:
    Interpreter(std::vector<AstInstruction::Ptr> instructions)
        : m_instructions(instructions) {}
    void run_code();
    void run_simulation();

   private:
    State m_state { };
    std::vector<AstInstruction::Ptr> m_instructions;
};
