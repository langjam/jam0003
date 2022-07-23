#pragma once

#include "ast/state.h"

class AstInstruction {
   public:
    AstInstruction() {}
    virtual ~AstInstruction() {}
    virtual void run(State* state) = 0;
};
