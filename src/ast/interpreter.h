#pragma once

#include <vector>

#include "ast/exprs/expr.h"
#include "ast/instructions/instruction.h"

class Interpreter {
   private:
    State m_state;
   public:
    void run(std::vector<AstInstruction*> instructions);
};
