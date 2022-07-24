#pragma once

#include <memory>

class AstInstruction {
   public:
    typedef std::shared_ptr<AstInstruction> Ptr;
    AstInstruction() {}
    virtual ~AstInstruction() {}
};
