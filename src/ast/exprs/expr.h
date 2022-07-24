#pragma once

#include <memory>

class AstExpr {
   public:
    typedef std::shared_ptr<AstExpr> Ptr;
    AstExpr() {}
    virtual ~AstExpr() {}
};
