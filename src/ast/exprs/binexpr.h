#pragma once

#include "expr.h"

class BinExpr : public AstExpr {
public:
    BinExpr(AstExpr* lhs, AstExpr* rhs)
        : AstExpr(), m_lhs(lhs), m_rhs(rhs) { }

protected:
    AstExpr* lhs() { return m_lhs; }
    AstExpr* rhs() { return m_rhs; }

private:
    AstExpr *m_lhs, *m_rhs;
};
