#pragma once

#include <ast/exprs/expr.h>

class BinExpr : public AstExpr {
   public:
    BinExpr(AstExpr::Ptr lhs, AstExpr::Ptr rhs) : AstExpr(), m_lhs(lhs), m_rhs(rhs) {}

   protected:
    AstExpr::Ptr lhs() { return m_lhs; }
    AstExpr::Ptr rhs() { return m_rhs; }

   private:
    AstExpr::Ptr m_lhs, m_rhs;
};
