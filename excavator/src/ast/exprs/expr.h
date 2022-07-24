#pragma once

#include <memory>

#include <runtime/values/value.h>

class State;

class AstExpr {
   public:
    typedef std::shared_ptr<AstExpr> Ptr;
    AstExpr() {}
    virtual ~AstExpr() {}

    virtual Value::Ptr eval(State& state) = 0;
};
